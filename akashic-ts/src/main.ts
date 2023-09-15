import {JoinEvent} from "@akashic/akashic-engine";

function main(param: any): void {
    console.log(g.game.selfId)

    const scene = new g.Scene({
        game: g.game,
        // このシーンで利用するアセットのIDを列挙し、シーンに通知します
        assetIds: ["player", "shot", "se", "font", "font_glyphs"]
    });

    let streamerId: undefined | string = undefined;
    g.game.onJoin.add((joinEvent) => {
        console.log(joinEvent.player.id)
        streamerId = joinEvent.player.id
    })
    console.log(g.game.selfId);

    console.log(param)
    scene.onLoad.add(() => {
        // ここからゲーム内容を記述します
// 上で生成した font.png と font_glyphs.json に対応するアセットを取得
        const fontAsset = g.game.scene().asset.getImageById("font");
        const fontGlyphAsset = g.game.scene().asset.getTextById("font_glyphs");

// テキストアセット (JSON) の内容をオブジェクトに変換
        const glyphInfo = JSON.parse(fontGlyphAsset.data);
        console.log(glyphInfo)
// ビットマップフォントを生成
        const font = new g.BitmapFont({
            src: fontAsset,
            glyphInfo: glyphInfo,

        });

        const label = new g.Label({
            font, scene: scene, text: "あかさたな",

        });

        label.scene.append(label)
        // 各アセットオブジェクトを取得します
        const playerImageAsset = scene.asset.getImageById("player");
        const shotImageAsset = scene.asset.getImageById("shot");
        const seAudioAsset = scene.asset.getAudioById("se");

        // 自分のID
        console.log(g.game.selfId);


        g.game.onJoin.add((event) => {
        })


        seAudioAsset.play();
        g.game.onJoin.add((event: JoinEvent) => {
            event.player.id
            console.log(event.eventFlags)
        })

        // プレイヤーを生成します
        const player = new g.Sprite({
            scene: scene,
            src: playerImageAsset,
            width: playerImageAsset.width,
            height: playerImageAsset.height
        });

        // プレイヤーの初期座標を、画面の中心に設定します
        player.x = (g.game.width - player.width) / 2;
        player.y = (g.game.height - player.height) / 2;

        player.onUpdate.add(() => {
            // 毎フレームでY座標を再計算し、プレイヤーの飛んでいる動きを表現します
            // ここではMath.sinを利用して、時間経過によって増加するg.game.ageと組み合わせて
            player.y = (g.game.height - player.height) / 2 + Math.sin(g.game.age % (g.game.fps * 10) / 4) * 10;

            // プレイヤーの座標に変更があった場合、 modified() を実行して変更をゲームに通知します
            player.modified();
        });

        scene.asset.getAllTexts()

        // 画面をタッチしたとき、SEを鳴らします
        scene.onPointDownCapture.add(() => {
            seAudioAsset.play();
            scene.asset.getAllTexts()
            // プレイヤーが発射する弾を生成します
            const shot = new g.Sprite({
                scene: scene,
                src: shotImageAsset,
                width: shotImageAsset.width,
                height: shotImageAsset.height
            });

            // 弾の初期座標を、プレイヤーの少し右に設定します
            shot.x = player.x + player.width;
            shot.y = player.y;
            shot.onUpdate.add(() => {
                // 毎フレームで座標を確認し、画面外に出ていたら弾をシーンから取り除きます
                if (shot.x > g.game.width) shot.destroy();

                // 弾を右に動かし、弾の動きを表現します
                shot.x += 10;
                // 変更をゲームに通知します
                shot.modified();
            });
            scene.append(shot);
        });
        scene.append(player);
        // ここまでゲーム内容を記述します
    });
    g.game.pushScene(scene);
}

export = main;
