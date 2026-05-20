#!/bin/bash
set -euo pipefail

cd /target
cargo build --quiet

pytest -q /target/tests > /logs/verifier/test_output.txt 2>&1 || true

python3 /workspace/log_parsers.py /logs/verifier/test_output.txt /logs/verifier/reward.json
