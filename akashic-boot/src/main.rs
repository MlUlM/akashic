use std::fs;
use std::path::Path;
use std::process::Command;

use clap::{arg, Parser};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    release: bool,

    #[arg(short, long)]
    target: Option<String>,

    #[arg(short, long)]
    example: Option<String>,
}


fn main() {
    let args = Args::parse();
    build(&args.example, args.release);
    wasm_bindgen(&args.example, args.release);
    convert_to_main_js();

    move_akashic_wasm();
    remove_out_dir();
    if let Some(target) = args.target {
        match target.as_str() {
            "serve" => akashic_serve(),
            "sandbox" => akashic_sandbox(),
            _ => {}
        }
    }
}


fn build(example: &Option<String>, release: bool) {
    let mut cmd = Command::new("cargo");
    cmd
        .arg("build")
        .args(["--target", "wasm32-unknown-unknown"]);

    if let Some(example) = example.as_ref() {
        cmd.args(["--example", example.as_str()]);
    }
    if release {
        cmd.arg("--release");
    }

    cmd.status().unwrap();
}


fn wasm_bindgen(example: &Option<String>, release: bool) {
    let wasm_path = if let Some(example) = example.as_ref() {
        format!("target/wasm32-unknown-unknown/{}/examples/{}.wasm", if release { "release" } else { "debug" }, example)
    } else {
        fs::read_dir(format!("target/wasm32-unknown-unknown/{}", if release { "release" } else { "debug" })).expect("Please build")
            .filter_map(|file| file.ok())
            .find_map(|file| {
                let file_path = file.path().to_str()?.to_string();
                let path = Path::new(&file_path);
                let ext = path.extension()?;
                if ext.to_str().is_some_and(|ext| ext == "wasm") {
                    return Some(file_path);
                }
                None
            })
            .expect("Not found wasm file")
    };


    Command::new("wasm-bindgen")
        .args(["--target", "no-modules"])
        .args(["--out-dir", "out"])
        .args(["--out-name", "akashic"])
        .arg("--no-typescript")
        .arg(wasm_path.clone())
        .status()
        .unwrap();
}


fn convert_to_main_js() {
    let akashic_js = fs::read_to_string("out/akashic.js").unwrap();
    let akashic_js = akashic_js
        .replace("let script_src;", "let script_src = \"\"")
        .replace("input = fetch(input)", "input = fetch(g.game._assetManager.configuration.wasm.path)")
        .replace("Object.assign(__wbg_init, { initSync }, __exports);", "Object.assign(__wbg_init, { initSync }, __exports)();");

    fs::write("akashic/script/main.js", format!(r#"
        class CustomSprite extends g.Sprite {{
            constructor(params) {{
                super(params);
                this.drawer = params.drawer;
            }}

            renderSelf(renderer, camera) {{
                this.drawer.render();
                super.renderSelf(renderer, camera);
            }}
        }}

        module.exports = () => {{
            g.isNode = () => (typeof window == 'undefined')
            g.canvas_only = (width, height) => {{
                const surface = g.game.resourceFactory.createSurface(width, height)
                return surface
            }}
            g.canvas = (width, height) => {{
                const surface = g.game.resourceFactory.createSurface(width, height)
                const gl = surface._drawable.getContext("webgl2")

                const sprite = new g.Sprite({{
                    scene: g.game.scene(),
                    src: surface,
                    anchorX: 0.5,
                    anchorY: 0.5,
                    x: g.game.width / 2,
                    y: g.game.height / 2,
                    width,
                    height
                }})
                g.game.scene().append(sprite)
                sprite.modified()

                return surface._drawable
            }}

            g.getEntityProperties = (entity) => ({{
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
            }})

            g.feedFilledRectProperties = (entity, cssColor) => {{
                entity.cssColor = cssColor
                entity.modified()
            }}

            g.feedLabelProperties = (entity, text, textAlign, textColor, widthAutoAdjust) => {{
                entity.text = text
                entity.textAlign = textAlign
                entity.textColor = textColor
                entity.widthAutoAdjust = widthAutoAdjust
                entity.invalidate()
            }}

            const halfWidth =  g.game.width / 2
            const halfHeight = g.game.height / 2

            g.feedEntityProperties = (entity, x, y, angle, width, height, scaleX, scaleY, anchorX, anchorY, touchable, visible) => {{
                entity.angle = angle;
                entity.resizeTo(width, height)
                const parent = entity.parent

                if(parent && !(parent instanceof g.Scene)){{
                    entity.moveTo(x + parent.width / 2, parent.height / 2 - y)
                }} else {{
                    entity.moveTo(x + halfWidth, halfHeight - y)
                }}

                entity.scale(scaleX, scaleY)
                entity.anchor(anchorX, anchorY)
                entity.touchable = touchable
                if(visible && !entity.visible()){{
                    entity.show()
                }} else if(!visible && entity.visible()){{
                    entity.hide()
                }}
                entity.modified()
            }}

            if (typeof window == 'undefined') {{
               globalThis.crypto = {{
                    getRandomValues: (args) => new Uint8Array(args.map(_ => Math.floor(g.game.random.generate() * 255)))
               }}
            }}

            const scene = new g.Scene({{
              game: g.game,
              assetPaths: [
                "/image/*",
                "/script/*",
                "/audio/*",
                "/text/*"
              ]
            }})

            scene.onLoad.addOnce(() => {{
                if (typeof window == 'undefined'){{
                    return;
                }}

                g.create_3d = (param) => new CustomSprite({{
                    scene,
                    src: param.src,
                    drawer: param.drawer
                }})

                {akashic_js}
            }})

            g.game.pushScene(scene)
        }}
    "#))
        .unwrap();
}


fn move_akashic_wasm() {
    fs::rename("out/akashic_bg.wasm", "akashic/script/akashic.wasm").unwrap();
}


fn remove_out_dir() {
    fs::remove_dir_all("out").unwrap();
}


fn akashic_serve() {
    Command::new("npx.cmd")
        .arg("akashic")
        .arg("serve")
        .arg("akashic")
        .args(["--target-service", "nicolive"])
        .status()
        .unwrap();
}


fn akashic_sandbox() {
    Command::new("npx.cmd")
        .arg("akashic-sandbox")
        .arg("akashic")
        .status()
        .unwrap();
}