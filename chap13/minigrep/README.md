# minigrep console app

Search text in file.

## Running

Case-sensitive search

```sh
cargo run -- frog poem.txt
```

Case-insensitive search

```sh
IGNORE_CASE=1 cargo run -- frog poem.txt
```