cargo build --release\
&& wasm-gc target/wasm32-unknown-unknown/release/cart.wasm\
&& w4 bundle ./target/wasm32-unknown-unknown/release/cart.wasm --html "build/ICallUponTheBEES.html" --title "I Call Upon the BEES!" --description "A bee genre 2D platformer."