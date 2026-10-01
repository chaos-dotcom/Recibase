#!/usr/bin/env python3
"""Before/after: build and test performance of the Scala original vs the Rust port.

Both sides start from the same kind of state:
  cold  = no build outputs (target/) and no compiled-artifact cache
          (Scala: ~/Library/Caches/sbt/v2, sbt 2's action/CAS cache - it makes a
           `clean` free, so it is cleared to match `cargo clean`)
          (dependency jars and the cargo registry cache are kept on both sides)
  warm  = run again with nothing changed

Commands are the ones that actually execute the tests: sbt 2's `test` runs
`testQuick` and skips unchanged suites, so the Scala side uses `testOnly *`.
"""
import argparse, json, os, re, shutil, subprocess, sys, time

WS = os.path.dirname(os.path.abspath(__file__))
SCALA = os.path.join(WS, 'Recibase')
RUST = os.path.join(WS, 'recibase-rs')
SBT_CACHE = os.path.expanduser('~/Library/Caches/sbt/v2')
JAVA_HOME = '/opt/homebrew/opt/openjdk@25'
SBT = '/opt/homebrew/bin/sbt'
LOG = os.path.join(WS, 'measure2.log')

def clear_scala():
    shutil.rmtree(os.path.join(SCALA, 'target'), ignore_errors=True)
    shutil.rmtree(SBT_CACHE, ignore_errors=True)

def clear_rust():
    shutil.rmtree(os.path.join(RUST, 'target'), ignore_errors=True)

def run(cmd, cwd, log):
    env = dict(os.environ)
    env['JAVA_HOME'] = JAVA_HOME
    env['PATH'] = JAVA_HOME + '/bin:' + env.get('PATH', '')
    start = time.time()
    with open(log, 'ab') as fh:
        fh.write(f'\n\n===== {time.strftime("%H:%M:%S")} $ {" ".join(cmd)} (cwd={cwd})\n'.encode())
        proc = subprocess.run(cmd, cwd=cwd, env=env, stdout=fh, stderr=subprocess.STDOUT, timeout=3600)
    return time.time() - start, proc.returncode

def analyse(log, marker):
    text = open(log, errors='replace').read()
    chunk = text[text.rfind(marker):]
    sbt_tasks = [float(x) for x in re.findall(r'\[success\] elapsed time: ([0-9.]+) s', chunk)]
    cargo_bins = [float(x) for x in re.findall(r'finished in ([0-9.]+)s', chunk)]
    scala_pass = re.findall(r'Passed: Total (\d+), Failed (\d+), Errors (\d+), Passed (\d+), Pending (\d+)', chunk)
    rust_pass = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed', chunk)
    return {
        'sbt_task_seconds': sbt_tasks[-1] if sbt_tasks else None,
        'cargo_test_seconds_total': round(sum(cargo_bins), 3) if cargo_bins else None,
        'scala_tests': (int(scala_pass[-1][3]), int(scala_pass[-1][0])) if scala_pass else None,
        'rust_tests_passed': sum(int(r[0]) for r in rust_pass) if rust_pass else 0,
        'rust_test_binaries': len(rust_pass),
    }

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--only', default='both')
    ap.add_argument('--repeat', type=int, default=1)
    args = ap.parse_args()
    open(LOG, 'w').close()
    results = []

    def measure(name, cmd, cwd, clear=None):
        if clear:
            clear()
        marker = cmd[0] + ' ' + cmd[-1]
        wall, rc = run(cmd, cwd, LOG)
        entry = {'name': name, 'command': ' '.join(cmd), 'cleared': bool(clear),
                 'wall_seconds': round(wall, 2), 'exit_code': rc, **analyse(LOG, marker)}
        print(json.dumps(entry), flush=True)
        results.append(entry)

    for _ in range(args.repeat):
        if args.only in ('both', 'scala'):
            measure('scala-cold-compile', [SBT, '--batch', '--server', 'compile'], SCALA, clear=clear_scala)
            measure('scala-cold-test', [SBT, '--batch', '--server', 'testOnly *'], SCALA, clear=clear_scala)
            measure('scala-warm-test', [SBT, '--batch', '--server', 'testOnly *'], SCALA)
        if args.only in ('both', 'rust'):
            measure('rust-cold-build', ['cargo', 'build', '--workspace'], RUST, clear=clear_rust)
            measure('rust-cold-test', ['cargo', 'test', '--workspace'], RUST, clear=clear_rust)
            measure('rust-warm-test', ['cargo', 'test', '--workspace'], RUST)
            measure('rust-cold-test-release', ['cargo', 'test', '--workspace', '--release'], RUST, clear=clear_rust)
            measure('rust-warm-test-release', ['cargo', 'test', '--workspace', '--release'], RUST)

    path = os.path.join(WS, 'results2.json')
    json.dump(results, open(path, 'w'), indent=1)
    print('wrote', path)

if __name__ == '__main__':
    main()
