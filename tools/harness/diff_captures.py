#!/usr/bin/env python3
"""Byte-compare two capture directories produced by capture.py."""
import argparse, os, sys

def load(d):
    out = {}
    for name in sorted(os.listdir(d)):
        if name.endswith('.norm'):
            out.setdefault(name.split('_', 1)[1][:-5], {})['norm'] = open(os.path.join(d, name), 'rb').read()
        elif name.endswith('.raw'):
            out.setdefault(name.split('_', 1)[1][:-4], {})['raw'] = open(os.path.join(d, name), 'rb').read()
    return out

def first_diff(a, b):
    n = min(len(a), len(b))
    for i in range(n):
        if a[i] != b[i]:
            return i
    return n if len(a) != len(b) else -1

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('reference')
    ap.add_argument('candidate')
    ap.add_argument('--show', type=int, default=8)
    ap.add_argument('--quiet', action='store_true')
    args = ap.parse_args()
    ref, cand = load(args.reference), load(args.candidate)
    names = sorted(set(ref) | set(cand))
    same = diff = missing = 0
    failures = []
    for name in names:
        if name not in cand:
            missing += 1; failures.append((name, 'missing from candidate', None)); continue
        if name not in ref:
            missing += 1; failures.append((name, 'extra in candidate', None)); continue
        if ref[name]['norm'] == cand[name]['norm']:
            same += 1
        else:
            diff += 1
            i = first_diff(ref[name]['norm'], cand[name]['norm'])
            failures.append((name, f'first difference at byte {i}', (ref[name]['norm'], cand[name]['norm'], i)))
    print(f'byte-identical (Date-normalised): {same}/{len(names)}   differing: {diff}   missing/extra: {missing}')
    for name, why, detail in failures[: args.show]:
        print(f'  FAIL {name}: {why}')
        if detail and not args.quiet:
            ref_b, cand_b, i = detail
            lo = max(0, i - 60); hi = i + 60
            print(f'    ref  ...{ref_b[lo:hi]!r}')
            print(f'    cand ...{cand_b[lo:hi]!r}')
    if len(failures) > args.show:
        print(f'  ... and {len(failures) - args.show} more')
    return 0 if not failures else 1

if __name__ == '__main__':
    sys.exit(main())
