all: 
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/reclaim_cosmwasm.wasm reclaim_cosmwasm.wasm

secret: 
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --no-default-features --features secret
	cp target/wasm32-unknown-unknown/release/reclaim_cosmwasm.wasm reclaim_cosmwasm.wasm

vanilla:
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --no-default-features --features vanilla
	cp target/wasm32-unknown-unknown/release/reclaim_cosmwasm.wasm reclaim_cosmwasm.wasm


clean:
	cargo clean
