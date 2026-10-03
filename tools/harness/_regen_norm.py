"""One-off: rewrite every capture dir's `.norm` from its committed `.raw`.

Needed after `capture.py`'s `normalise()` gained the manifest `version` rule, so
the committed references carry the same normalisation the verifier applies.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from capture import normalise

here = os.path.dirname(os.path.abspath(__file__))
rewritten = 0
for directory in sorted(glob.glob(os.path.join(here, "capture-scala*"))):
    if not os.path.isdir(directory):
        continue
    for raw_path in sorted(glob.glob(os.path.join(directory, "*.raw"))):
        with open(raw_path, "rb") as fh:
            raw = fh.read()
        with open(raw_path[:-4] + ".norm", "wb") as fh:
            fh.write(normalise(raw))
        rewritten += 1
print(f"rewrote {rewritten} .norm files")
