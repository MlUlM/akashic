const fs = require("fs")
const js = fs.readFileSync("out/bevy-akashic.js")
fs.copyFileSync("out/bevy-akashic_bg.wasm", "akashic/script/bevy-akashic.wasm")

const mainJsPath = "./akashic/script/main.js"
const wasmCode = js
    .toString()
    .replace("let script_src;", "let script_src = \"\";")
    .replace("input = fetch(input);", `
        if (typeof window == 'undefined') {
            input = fetch(g.game._assetManager.configuration.wasm.path)
        }else{
            input = fetch(g.game._assetManager.configuration.wasm.path)
        }
    `)
    .replace("Object.assign(__wbg_init, { initSync }, __exports);", "Object.assign(__wbg_init, { initSync }, __exports)();")

fs.writeFileSync(mainJsPath, `
   
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
        const scene = new g.Scene({
          game: g.game
        })
        g.game.pushScene(scene)
        
        g.Sprite.prototype.updateAll = function(x, y, angle, width, height){
            this.x = x;
            this.y = y;
            this.angle = angle;
            this.width = width;
            this.height = height;
            this.modified();
        }

        if (typeof window == 'undefined') {
            globalThis.crypto = {
                getRandomValues: (args) => new Uint8Array(args.map(_ => Math.floor(g.game.random.generate() * 255)))      
            }
        }else{
            
            g.game.renderers[0].surface.canvas.id = "bevy"
        }
         
        ${wasmCode}
    }
`)
