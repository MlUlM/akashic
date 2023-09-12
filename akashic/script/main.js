module.exports = () => {
    const scene = new g.Scene({
        game: g.game,
        // このシーンで利用するアセットのIDを列挙し、シーンに通知します
        assetIds: ["player", "shot", "se", "font", "font_glyphs"]
    });


    scene.onLoad.add(() => {
        const fontAsset = scene.asset.getImageById("font");
        const fontGlyphAsset = scene.asset.getTextById("font_glyphs");
        const glyphInfo = JSON.parse(fontGlyphAsset.data);

        const font = new g.BitmapFont({
            src: fontAsset,
            glyphInfo: glyphInfo
        });
        scene.append(font)
    })
    g.game.pushScene(scene)
}