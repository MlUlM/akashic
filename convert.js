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
    
        g.Sprite.prototype.updateAll = function(x, y, angle, width, height){
            this.x = x;
            this.y = y;
            this.angle = angle;
            this.width = width;
            this.height = height;
            this.modified();
            // g.game.modified(true);
        }

    if (typeof window == 'undefined') {
        globalThis.crypto = {
            getRandomValues: (args) => new Uint8Array(args.map(_ => Math.floor(g.game.random.generate() * 255)))
                
       }
    }
    
    ${wasmCode}
       
    }
`)
