# aleph2py

Standalone transpiler from Aleph to Python.
Reads Aleph source from stdin, writes Python to stdout.

## Prerequisites

- Rust installed on your system

## Build & run

```bash
cargo build --release
echo "3 + 4" | ./target/release/aleph2py
```

Output:

```python
3 + 4
```

## Pipeline

```
stdin → aleparser → constantfolding → pythongen → stdout
```

## Related

- [`aleparser`](https://github.com/aleph-lang/aleparser) — Aleph parser
- [`pythongen`](https://github.com/aleph-lang/pythongen) — Python code generator
- [`alephc`](https://github.com/aleph-lang/aleph) — full compiler with multiple input/output targets
