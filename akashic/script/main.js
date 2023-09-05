// 描画時に呼び出されるコールバックを指定できるスプライトクラス
class CustomSprite extends g.Sprite {
    constructor(params) {
        super(params);
        this.drawer = params.drawer;
    }

    renderSelf(renderer, camera) {
        this.drawer();
        super.renderSelf(renderer, camera);
    }
}

module.exports = () => {
    var scene = new g.Scene({
        game: g.game
    });
    g.game.pushScene(scene);

    scene.onLoad.addOnce(() => {
        // サーフェース作成
        const src = g.game.resourceFactory.createSurface(100, 200);
        // WebGL2作成
        const gl = src._drawable.getContext('webgl2');

        // シェーダー作成
        const shader = gl.createProgram();
        const vs = gl.createShader(gl.VERTEX_SHADER);
        const fs = gl.createShader(gl.FRAGMENT_SHADER);
        gl.shaderSource(vs, '#version 300 es\n' +
            'uniform float angle;' +
            'out vec3 color;' +
            'void main(void){' +
            '   gl_Position = vec4(sin(float(gl_VertexID) * 2.094 + angle), cos(float(gl_VertexID) * 2.094 + angle), 1., 1.);' +
            '   color = vec3(gl_VertexID == 0 ? 1.0 : 0.0, gl_VertexID == 1 ? 1.0 : 0.0, gl_VertexID == 2 ? 1.0 : 0.0);' +
            '}');
        gl.shaderSource(fs, '#version 300 es\n' +
            'precision mediump float;' +
            'in vec3 color;' +
            'out vec4 result;' +
            'void main(void){' +
            '   result = vec4(color, 1.0);' +
            '}');
        gl.compileShader(vs);
        gl.compileShader(fs);
        gl.attachShader(shader, vs);
        gl.attachShader(shader, fs);
        gl.linkProgram(shader);

        // シェーダーパラメータ（ユニフォームロケーション）取得
        const angle = gl.getUniformLocation(shader, 'angle');

        // レンダリング結果表示用エンティティ作成
        const sprite = new CustomSprite({
            scene,
            src,
            parent: scene,
            anchorX: 0.5,
            anchorY: 0.5,
            drawer: () => {
                // WebGL描画処理（画面クリアと三角描画のみ）
                gl.clearColor(0, 0, 0, 1);
                gl.clear(gl.COLOR_BUFFER_BIT);

                gl.useProgram(shader);
                gl.uniform1f(angle, g.game.age * 0.02);
                gl.drawArrays(gl.TRIANGLE_STRIP, 0, 3);
            }
        });
        // 適当にくるくる回しとく
        sprite.onUpdate.add(() => {
            sprite.x = Math.cos(g.game.age * 0.02) * 100 + g.game.width * 0.5;
            sprite.y = Math.sin(g.game.age * 0.02) * 100 + g.game.height * 0.5;
            sprite.modified();
        });

        // 後片付け
        scene.onStateChange.addOnce(state => {
            if (state == 'before-destroyed') {
                gl.deleteShader(vs);
                gl.deleteShader(fs);
                gl.deleteProgram(shader);
            }
        });
    });
};