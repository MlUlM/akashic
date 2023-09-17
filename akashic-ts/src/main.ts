import {BodyType, Box2D} from "@akashic-extension/akashic-box2d";

function main(param: any): void {
    console.log(g.game.selfId)

    const scene = new g.Scene({
        game: g.game,
        // このシーンで利用するアセットのIDを列挙し、シーンに通知します
        assetIds: ["player", "shot", "se", "font", "font_glyphs"]
    });

    scene.onLoad.add(() => {
        const box2d = new Box2D({
            gravity: [0, 9.8],
            scale: 50,
            sleep: true
        });

        const ground = new g.FilledRect({
            scene: scene,
            cssColor: "red",
            width: 1000,
            height: 20,
            anchorX: 0.5,
            anchorY: 0.5,
            x: g.game.width / 2,
            y: g.game.height / 2
        });
        scene.append(ground);
        ground.modified();

        const entityFixDef = box2d.createFixtureDef({
            density: 1.0, // 密度
            friction: 0.5, // 摩擦係数
            restitution: 0.3, // 反発係数
            shape: box2d.createRectShape(ground.width, ground.height)// 形状
        });
        const entityDef = box2d.createBodyDef({
            type: BodyType.Static,
            gravityScale: 0.5
        });
        box2d.createBody(ground, entityDef, entityFixDef);

        let num = 30;
        let rad = 10.0;

        let shift = rad * 2.0 + rad;
        let centerx = shift * (num / 2);
        let centery = shift / 2.0;

        let offset = -num * (rad * 2.0 + rad) * 0.5;

        for (let j = 0; j < 20; j++) {
            for (let i = 0; i < num; i++) {
                let x = i * shift - centerx + offset + g.game.width * 0.8;
                let y = g.game.height / 2 - (j * shift + centery + 30.0);
                let entity = new g.FilledRect({
                    cssColor: "blue",
                    x,
                    y,
                    width: rad * 2,
                    height: rad * 2,
                    scene
                });
                scene.append(entity);
                entity.modified();

                const entityFixDef = box2d.createFixtureDef({
                    density: 1.0, // 密度
                    friction: 0.5, // 摩擦係数
                    restitution: 0.3, // 反発係数
                    shape: box2d.createRectShape(entity.width, entity.height) // 形状
                });
                const entityDef = box2d.createBodyDef({
                    type: BodyType.Dynamic
                });
                box2d.createBody(entity, entityDef, entityFixDef);

            }

            offset -= 0.05 * rad * (num - 1.0);
        }
        scene.onUpdate.add(() => {
            // 物理エンジンの世界をすすめる
            box2d.step(1 / g.game.fps);
        });
    })

    g.game.pushScene(scene)
}


export = main;
