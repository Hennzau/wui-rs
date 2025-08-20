b:
    cargo build

br:
    cargo build --release

x example:
    cargo run --example {{example}}

xr example:
    cargo run --release --example {{example}}

c:
    cargo check

cl:
    cargo clippy --fix --allow-dirty --allow-staged
