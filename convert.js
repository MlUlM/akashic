const fs = require("fs")
const js = fs.readFileSync("out/bevy-akashic.js")


const l = `
const process = undefined;
const Buffer = undefined;
const setImmediate = undefined;
`

const js2 = l + js.toString()
const js3 = js2.replace("let script_src;", "let script_src = g.game._assetManager.configuration.wasm.path;")
const js4 = js3.replace("wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);", "wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);module.exports = {wasm_bindgen,  locateFile: path => g.game._assetManager.configuration.wasm.path};")
fs.writeFileSync("./akashic/script/game.js", js4)
fs.renameSync("out/bevy-akashic_bg.wasm", "./akashic/script/game_bg.wasm")