const fs = require("fs")
const js = fs.readFileSync("out/bevy-akashic.js")

const mainJsPath = "./akashic/script/main.js"
const wasmCode = js
    .toString()
    .replace("let script_src;", "let script_src = \"\";")
    .replace("input = fetch(input);", "input = base64ToArrayBuffer(wasmBase64);")
    .replace("wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);", "Object.assign(__wbg_init, { initSync }, __exports)();")

fs.writeFileSync(mainJsPath, `
    const wasmBase64 = \"${Buffer.from(fs.readFileSync("out/bevy-akashic_bg.wasm")).toString("base64")}\"
    
    function base64ToArrayBuffer(base64) {
        var binaryString = atob(base64);
        var bytes = new Uint8Array(binaryString.length);
        for (var i = 0; i < binaryString.length; i++) {
            bytes[i] = binaryString.charCodeAt(i);
        }
        return bytes.buffer;
    }
    
    // main function
    module.exports = () => {
        ${wasmCode}
    }
`)
