const fs = require("fs")
const js = fs.readFileSync("out/state.js")
fs.copyFileSync("out/state_bg.wasm", "akashic/script/akashic.wasm")

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
    module.exports = () => {          
        g.getEntityProperties = (entity) => ({
            id: entity.id,
            x: entity.x,
            y: entity.y,
            width: entity.width,
            height: entity.height,
            angle: entity.angle,
            scaleX: entity.scaleX,
            scaleY: entity.scaleY,
            anchorX: entity.anchorX,
            anchorY: entity.anchorY,
            touchable: entity.touchable,
            visible: entity.visible()
        })
        
        g.feedFilledRectProperties = (entity, cssColor) => {
            entity.cssColor = cssColor
            entity.modified()
        }
        
        g.feedLabelProperties = (entity, text, textAlign, textColor, widthAutoAdjust) => {
            entity.text = text
            entity.textAlign = textAlign
            entity.textColor = textColor
            entity.widthAutoAdjust = widthAutoAdjust
            entity.invalidate()
        }
        
        g.feedEntityProperties = (entity, x, y, angle, width, height, scaleX, scaleY, anchorX, anchorY, touchable, visible) => {
            entity.angle = angle;
            entity.resizeTo(width, height)
            entity.moveTo(x, y)
            entity.scale(scaleX, scaleY)
            entity.anchor(anchorX, anchorY)
            entity.touchable = touchable
            if(visible && !entity.visible()){
                entity.show()
            }else if(!visible && entity.visible()){
                entity.hide()
            }
            entity.modified()
        }
         
        if (typeof window == 'undefined') {
           globalThis.crypto = {
                getRandomValues: (args) => new Uint8Array(args.map(_ => Math.floor(g.game.random.generate() * 255)))      
           }
        }else{
            globalThis.crypto.getRandomValues = (args) => new Uint8Array(args.map(_ => Math.floor(g.game.random.generate() * 255)))   
        }
        
        
        const scene = new g.Scene({
          game: g.game,
          assetPaths: [
            "/image/*",
            "/script/*",
            "/audio/*",
            "/text/*"
          ]
        })
        
        scene.onLoad.addOnce(() => {
            ${wasmCode}
        })

        g.game.pushScene(scene)
    }
`)
