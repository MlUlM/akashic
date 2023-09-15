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
        
        // g.E.prototype.getProperties = function(){
        //     id: this.id,
        //     x: this.x,
        //     y: this.y,
        //     width: this.width,
        //     height: this.height,
        //     angle: this.angle
        // }
        
        g.updateText = (entity, text, textAlign, textColor, widthAutoAdjust) => {
            entity.text = text
            entity.textAlign = textAlign
            entity.textColor = textColor
            entity.widthAutoAdjust = widthAutoAdjust
        }
        
        g.updateEntityBase = (entity, x, y, angle, width, height) => {
            entity.x = x;
            entity.y = y;
            entity.angle = angle;
            entity.width = width;
            entity.height = height;
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
