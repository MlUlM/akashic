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
        .arg(wasm_path)
        .status()
        .unwrap();
}


fn convert_to_main_js() {
    let akashic_js = fs::read_to_string("out/akashic.js").unwrap();
    let akashic_js = akashic_js
        .replace("let script_src;", "let script_src = \"\"")
        .replace("input = fetch(input)", "input = g.game.scene().asset.getBinaryData('/assets/script/akashic.wasm');")
        .replace("wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);", r#"
            const init = Object.assign(__wbg_init, {initSync}, __exports);
            module.exports = {
                init,
                pass: (passG) => {
                    g = passG
                }
            }
        "#);
    fs::write("assets/script/akashic.js", akashic_js).unwrap();
}


fn move_akashic_wasm() {
    fs::rename("out/akashic_bg.wasm", "assets/script/akashic.wasm").unwrap();
}


fn remove_out_dir() {
    fs::remove_dir_all("out").unwrap();
}


fn akashic_serve() {
    Command::new("npx.cmd")
        .arg("akashic")
        .arg("serve")
        .arg(".")
        .args(["-s", "nicolive:multi_admission"])
        .status()
        .unwrap();
}


fn akashic_sandbox() {
    Command::new("npx.cmd")
        .arg("akashic-sandbox")
        .arg(".")
        .status()
        .unwrap();
}