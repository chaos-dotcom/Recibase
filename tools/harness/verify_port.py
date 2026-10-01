#!/usr/bin/env python3
"""Start the Rust server, capture the same 114 requests, and diff byte-exactly.

  python3 verify_port.py --reference capture-scala --out capture-rust \
      [--env MEAL_LOG_CSV_URL=file:///...] [--binary target/release/recibase]
"""
import argparse, os, socket, subprocess, sys, time, json

WS = os.path.dirname(os.path.abspath(__file__))

def free_port():
    s = socket.socket(); s.bind(('127.0.0.1', 0)); port = s.getsockname()[1]; s.close(); return port

def wait_for(port, timeout=30):
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            with socket.create_connection(('127.0.0.1', port), timeout=0.5):
                return True
        except OSError:
            time.sleep(0.1)
    return False

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--reference', required=True)
    ap.add_argument('--out', required=True)
    ap.add_argument('--binary', required=True)
    ap.add_argument('--env', action='append', default=[])
    ap.add_argument('--show', type=int, default=12)
    ap.add_argument('--requests', default=os.path.join(WS, 'requests.json'))
    args = ap.parse_args()

    port = free_port()
    env = dict(os.environ)
    env['PORT'] = str(port)
    for item in args.env:
        key, _, value = item.partition('=')
        env[key] = value
    server = subprocess.Popen([args.binary], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    try:
        if not wait_for(port):
            print('server did not start; output:')
            print(server.stdout.read().decode(errors='replace')[-4000:])
            return 2
        subprocess.run([sys.executable, os.path.join(WS, 'capture.py'), '--port', str(port),
                        '--out', args.out, '--requests', args.requests],
                       check=True, stdout=subprocess.DEVNULL)
    finally:
        server.terminate()
        try:
            server.wait(timeout=10)
        except subprocess.TimeoutExpired:
            server.kill()
    return subprocess.run([sys.executable, os.path.join(WS, 'diff_captures.py'),
                           args.reference, args.out, '--show', str(args.show)]).returncode

if __name__ == '__main__':
    sys.exit(main())
