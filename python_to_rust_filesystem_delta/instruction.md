Translate the repository located at /source into an idiomatic Rust implementation in /target.

The Python repository implements a filesystem delta tool with three major components:

1. Snapshot: walk a directory and record file metadata, hashes, and symlink targets.
2. Delta: compute a patch describing the differences between two snapshots.
3. Apply: apply a patch to mutate a directory to match the target snapshot.

Your Rust translation must:

- preserve all behavior and semantics
- preserve deterministic ordering of operations
- preserve patch structure and JSON formats
- match the CLI behavior of the Python version
- build successfully with Cargo
- pass all tests executed by the verifier

The verifier will run:

    cd /target && cargo build --quiet
    pytest -q /target/tests

Your output must be a complete Rust repository under /target.
