const {execSync} = require("child_process");

// execSync("cargo build --target wasm32-unknown-unknown")
execSync("wasm-bindgen --target no-modules --out-dir out --no-typescript target/wasm32-unknown-unknown/release/examples/hello.wasm")
execSync("node convert.js")
execSync("akashic serve akashic --target-service nicolive")