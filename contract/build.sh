RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/auction.wasm ./res/auction.wasm

near login
# Setup master account for demo
ID_MASTER=master_01.testnet

