#!/usr/bin/env python3
"""Before/after test-performance protocol.

Runs, in order, and records wall-clock times plus the suite's own reported time:

  Scala  cold : rm -rf target; sbt test
  Scala  warm : sbt test
  Rust   cold : cargo clean; cargo test
  Rust   warm : cargo test
  Rust   cold (release) / warm (release)

Usage: python3 measure.py [--only scala|rust] [--repeat 1]
Results are appended to results.json and printed.
"""
import argparse, json, os, re, shutil, subprocess, sys, time

WS = os.path.dirname(os.path.abspath(__file__))
SCALA = os.path.join(WS, 'Recibase')
RUST = os.path.join(WS, 'recibase-rs')
JAVA_HOME = '/opt/homebrew/opt/openjdk@25'
SBT = '/opt/homebrew/bin/sbt'
LOG = os.path.join(WS, 'measure.log')

def run(cmd, cwd, env=None, log=LOG, timeout=3600):
    full_env = dict(os.environ)
    full_env['JAVA_HOME'] = JAVA_HOME
    full_env['PATH'] = JAVA_HOME + '/bin:' + full_env.get('PATH', '')
    if env:
        full_env.update(env)
    start = time.time()
    with open(log, 'ab') as fh:
        fh.write(f'\n\n===== {time.strftime("%H:%M:%S")} $ {" ".join(cmd)} (cwd={cwd})\n'.encode())
        proc = subprocess.run(cmd, cwd=cwd, env=full_env, stdout=fh, stderr=subprocess.STDOUT, timeout=timeout)
    wall = time.time() - start
    return wall, proc.returncode

def tail_reported_time():
    try:
        text = open(LOG, errors='replace').read()
    except OSError:
        return None
    sbt = re.findall(r'\[success\] elapsed time: ([0-9.]+) s', text)
    cargo = re.findall(r'finished in ([0-9.]+)s', text)
    return {'sbt_task_seconds': float(sbt[-1]) if sbt else None,
            'cargo_test_seconds': float(cargo[-1]) if cargo else None}

def suite_counts():
    text = open(LOG, errors='replace').read()
    scala = re.findall(r'Passed: Total (\d+), Failed (\d+), Errors (\d+), Passed (\d+), Pending (\d+)', text)
    rust = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed', text)
    return {
        'scala_last': scala[-1] if scala else None,
        'rust_passed_total': sum(int(r[0]) for r in rust) if rust else None,
    }

def measure(name, cmd, cwd, clean=None, env=None):
    if clean:
        target = os.path.join(cwd, clean)
        shutil.rmtree(target, ignore_errors=True)
    wall, rc = run(cmd, cwd, env=env)
    reported = tail_reported_time()
    entry = {'name': name, 'command': ' '.join(cmd), 'cwd': cwd, 'wall_seconds': round(wall, 2),
             'exit_code': rc, **reported, **suite_counts()}
    print(json.dumps(entry))
    return entry

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--only', default='both')
    ap.add_argument('--repeat', type=int, default=1)
    args = ap.parse_args()
    results = []
    open(LOG, 'w').close()

    for i in range(args.repeat):
        if args.only in ('both', 'scala'):
            results.append(measure(f'scala-cold-{i}', [SBT, '--batch', '--server', 'test'], SCALA, clean='target'))
            results.append(measure(f'scala-warm-{i}', [SBT, '--batch', '--server', 'test'], SCALA))
        if args.only in ('both', 'rust'):
            results.append(measure(f'rust-cold-{i}', ['cargo', 'test', '--workspace'], RUST, clean='target'))
            results.append(measure(f'rust-warm-{i}', ['cargo', 'test', '--workspace'], RUST))
            results.append(measure(f'rust-cold-release-{i}', ['cargo', 'test', '--workspace', '--release'], RUST, clean='target'))
            results.append(measure(f'rust-warm-release-{i}', ['cargo', 'test', '--workspace', '--release'], RUST))

    path = os.path.join(WS, 'results.json')
    existing = []
    if os.path.exists(path):
        existing = json.load(open(path))
    json.dump(existing + results, open(path, 'w'), indent=1)
    print('\nwrote', path)

if __name__ == '__main__':
    main()
