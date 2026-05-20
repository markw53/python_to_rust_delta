import json
import sys

def parse_log_pytest(log_path, out_path):
    with open(log_path, "r") as f:
        text = f.read()

    passed = "failed" not in text.lower()

    result = {
        "passed": passed,
        "details": text
    }

    with open(out_path, "w") as f:
        json.dump(result, f)

if __name__ == "__main__":
    parse_log_pytest(sys.argv[1], sys.argv[2])
