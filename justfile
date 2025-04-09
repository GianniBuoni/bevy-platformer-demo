run:
  just lint
  cargo run --features debug

release:
  just lint
  cargo run --release

check:
  cargo check

lint:
  just check
  cargo clippy

