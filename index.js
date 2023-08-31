const {execSync} = require("child_process");

execSync("cargo build --target wasm32-unknown-unknown")
execSync("wasm-bindgen --reference-types --target no-modules --out-dir out --no-typescript target/wasm32-unknown-unknown/debug/bevy-akashic.wasm")
execSync("node convert.js")