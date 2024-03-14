# grape

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

Custom implementation of infamous "grep" tool, written in Rust.

## compilation

Execute the following command to create a production-grade binary.

```bash
cargo build --release
```

and verify that `target/release/grape` has been created.

## execution

Execute the following command to search for `query` in `file.txt` file. 

```bash
./target/release/grape <query> <file.txt>
```