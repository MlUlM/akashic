
const process = undefined;
const Buffer = undefined;
const setImmediate = undefined;
let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src = g.game._assetManager.configuration.wasm.path;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm = undefined;

    const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

    if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

    let cachedUint8Memory0 = null;

    function getUint8Memory0() {
        if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
            cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_1.get(state.dtor)(a, state.b);

                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;

        return real;
    }
    function __wbg_adapter_22(arg0, arg1, arg2) {
        wasm.closure361_externref_shim(arg0, arg1, arg2);
    }

    function __wbg_adapter_25(arg0, arg1, arg2) {
        wasm.closure359_externref_shim(arg0, arg1, arg2);
    }

    function __wbg_adapter_28(arg0, arg1) {
        wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he58808aacd8a67f7(arg0, arg1);
    }

    let cachedInt32Memory0 = null;

    function getInt32Memory0() {
        if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
            cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachedInt32Memory0;
    }

    let WASM_VECTOR_LEN = 0;

    const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
        : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedUint32Memory0 = null;

    function getUint32Memory0() {
        if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
            cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
        }
        return cachedUint32Memory0;
    }

    function getArrayJsValueFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        const mem = getUint32Memory0();
        const slice = mem.subarray(ptr / 4, ptr / 4 + len);
        const result = [];
        for (let i = 0; i < slice.length; i++) {
            result.push(wasm.__wbindgen_export_0.get(slice[i]));
        }
        wasm.__externref_drop_slice(ptr, len);
        return result;
    }

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_export_0.set(idx, obj);
        return idx;
    }

    function passArrayJsValueToWasm0(array, malloc) {
        const ptr = malloc(array.length * 4, 4) >>> 0;
        const mem = getUint32Memory0();
        for (let i = 0; i < array.length; i++) {
            mem[ptr / 4 + i] = addToExternrefTable0(array[i]);
        }
        WASM_VECTOR_LEN = array.length;
        return ptr;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    let cachedFloat32Memory0 = null;

    function getFloat32Memory0() {
        if (cachedFloat32Memory0 === null || cachedFloat32Memory0.byteLength === 0) {
            cachedFloat32Memory0 = new Float32Array(wasm.memory.buffer);
        }
        return cachedFloat32Memory0;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    }
    /**
    */
    class FilledRectParameter {

        static __wrap(ptr) {
            ptr = ptr >>> 0;
            const obj = Object.create(FilledRectParameter.prototype);
            obj.__wbg_ptr = ptr;

            return obj;
        }

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;

            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_filledrectparameter_free(ptr);
        }
        /**
        * @returns {any}
        */
        get scene() {
            const ret = wasm.__wbg_get_filledrectparameter_scene(this.__wbg_ptr);
            return ret;
        }
        /**
        * @param {any} arg0
        */
        set scene(arg0) {
            wasm.__wbg_set_filledrectparameter_scene(this.__wbg_ptr, arg0);
        }
        /**
        * @returns {string}
        */
        get cssColor() {
            let deferred1_0;
            let deferred1_1;
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_filledrectparameter_cssColor(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getInt32Memory0()[retptr / 4 + 1];
                deferred1_0 = r0;
                deferred1_1 = r1;
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
                wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
            }
        }
        /**
        * @param {string} arg0
        */
        set cssColor(arg0) {
            const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.__wbg_set_filledrectparameter_cssColor(this.__wbg_ptr, ptr0, len0);
        }
        /**
        * @returns {number}
        */
        get width() {
            const ret = wasm.__wbg_get_filledrectparameter_width(this.__wbg_ptr);
            return ret;
        }
        /**
        * @param {number} arg0
        */
        set width(arg0) {
            wasm.__wbg_set_filledrectparameter_width(this.__wbg_ptr, arg0);
        }
        /**
        * @returns {number}
        */
        get height() {
            const ret = wasm.__wbg_get_filledrectparameter_height(this.__wbg_ptr);
            return ret;
        }
        /**
        * @param {number} arg0
        */
        set height(arg0) {
            wasm.__wbg_set_filledrectparameter_height(this.__wbg_ptr, arg0);
        }
        /**
        * @returns {boolean}
        */
        get touchable() {
            const ret = wasm.__wbg_get_filledrectparameter_touchable(this.__wbg_ptr);
            return ret !== 0;
        }
        /**
        * @param {boolean} arg0
        */
        set touchable(arg0) {
            wasm.__wbg_set_filledrectparameter_touchable(this.__wbg_ptr, arg0);
        }
    }
    __exports.FilledRectParameter = FilledRectParameter;
    /**
    */
    class Object2DParameterObject {

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;

            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_object2dparameterobject_free(ptr);
        }
        /**
        *このオブジェクトの横位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
        * @returns {number | undefined}
        */
        get x() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_x(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *このオブジェクトの横位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
        * @param {number | undefined} arg0
        */
        set x(arg0) {
            wasm.__wbg_set_object2dparameterobject_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * このオブジェクトの縦位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
        *     * @default 0
        *
        * @returns {number | undefined}
        */
        get y() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_y(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * このオブジェクトの縦位置。実際の座標位置はscaleX, scaleY, angle, anchorX, anchorYの値も考慮する必要がある。
        *     * @default 0
        *
        * @param {number | undefined} arg0
        */
        set y(arg0) {
            wasm.__wbg_set_object2dparameterobject_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * このオブジェクトの横幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
        *     * @default 0
        *
        * @returns {number | undefined}
        */
        get width() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_width(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * このオブジェクトの横幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
        *     * @default 0
        *
        * @param {number | undefined} arg0
        */
        set width(arg0) {
            wasm.__wbg_set_object2dparameterobject_width(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * このオブジェクトの縦幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
        *     * @default 0
        *
        * @returns {number | undefined}
        */
        get height() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_height(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * このオブジェクトの縦幅。実際の表示領域としてはscaleX, scaleY, angleの値も考慮する必要がある。
        *     * @default 0
        *
        * @param {number | undefined} arg0
        */
        set height(arg0) {
            wasm.__wbg_set_object2dparameterobject_height(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * 0～1でオブジェクトの不透明度を表す。
        *     * この値が0の場合、Rendererは描画処理を省略する。
        *     * @default 1
        *
        * @returns {number | undefined}
        */
        get opacity() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_opacity(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * 0～1でオブジェクトの不透明度を表す。
        *     * この値が0の場合、Rendererは描画処理を省略する。
        *     * @default 1
        *
        * @param {number | undefined} arg0
        */
        set opacity(arg0) {
            wasm.__wbg_set_object2dparameterobject_opacity(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * オブジェクトの横方向の倍率。
        *     * @default 1
        *
        * @returns {number | undefined}
        */
        get scaleX() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_scaleX(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * オブジェクトの横方向の倍率。
        *     * @default 1
        *
        * @param {number | undefined} arg0
        */
        set scaleX(arg0) {
            wasm.__wbg_set_object2dparameterobject_scaleX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * オブジェクトの縦方向の倍率。
        *     * @default 1
        *
        * @returns {number | undefined}
        */
        get scaleY() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_scaleY(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * オブジェクトの縦方向の倍率。
        *     * @default 1
        *
        * @param {number | undefined} arg0
        */
        set scaleY(arg0) {
            wasm.__wbg_set_object2dparameterobject_scaleY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * オブジェクトの回転。度数で指定する。
        *     * @default 0
        *
        * @returns {number | undefined}
        */
        get angle() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_angle(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        *
        *     * オブジェクトの回転。度数で指定する。
        *     * @default 0
        *
        * @param {number | undefined} arg0
        */
        set angle(arg0) {
            wasm.__wbg_set_object2dparameterobject_angle(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        *
        *     * 描画時の合成方法を指定する。
        *     * 省略された場合、合成方法を指定しない（親の合成方法を利用する）。
        *     * なお `CompositeOperation` での指定は非推奨である。 `CompositeOperationString` を利用すること。
        *     * @default undefined
        *
        * @returns {string | undefined}
        */
        get compositeOperation() {
            const ret = wasm.__wbg_get_object2dparameterobject_compositeOperation(this.__wbg_ptr);
            return ret;
        }
        /**
        *
        *     * 描画時の合成方法を指定する。
        *     * 省略された場合、合成方法を指定しない（親の合成方法を利用する）。
        *     * なお `CompositeOperation` での指定は非推奨である。 `CompositeOperationString` を利用すること。
        *     * @default undefined
        *
        * @param {string | undefined} arg0
        */
        set compositeOperation(arg0) {
            wasm.__wbg_set_object2dparameterobject_compositeOperation(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
        }
        /**
        * @returns {number | undefined}
        */
        get anchorX() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_anchorX(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        * @param {number | undefined} arg0
        */
        set anchorX(arg0) {
            wasm.__wbg_set_object2dparameterobject_anchorX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
        /**
        * @returns {number | undefined}
        */
        get anchorY() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_object2dparameterobject_anchorY(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getFloat32Memory0()[retptr / 4 + 1];
                return r0 === 0 ? undefined : r1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        * @param {number | undefined} arg0
        */
        set anchorY(arg0) {
            wasm.__wbg_set_object2dparameterobject_anchorY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
        }
    }
    __exports.Object2DParameterObject = Object2DParameterObject;
    /**
    */
    class SceneParameterObject {

        static __wrap(ptr) {
            ptr = ptr >>> 0;
            const obj = Object.create(SceneParameterObject.prototype);
            obj.__wbg_ptr = ptr;

            return obj;
        }

        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;

            return ptr;
        }

        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_sceneparameterobject_free(ptr);
        }
        /**
        * @returns {any}
        */
        get game() {
            const ret = wasm.__wbg_get_sceneparameterobject_game(this.__wbg_ptr);
            return ret;
        }
        /**
        * @param {any} arg0
        */
        set game(arg0) {
            wasm.__wbg_set_sceneparameterobject_game(this.__wbg_ptr, arg0);
        }
        /**
        * @returns {(string)[] | undefined}
        */
        get assetIds() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_sceneparameterobject_assetIds(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getInt32Memory0()[retptr / 4 + 1];
                let v1;
                if (r0 !== 0) {
                    v1 = getArrayJsValueFromWasm0(r0, r1).slice();
                    wasm.__wbindgen_free(r0, r1 * 4);
                }
                return v1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        * @param {(string)[] | undefined} arg0
        */
        set assetIds(arg0) {
            var ptr0 = isLikeNone(arg0) ? 0 : passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.__wbg_set_sceneparameterobject_assetIds(this.__wbg_ptr, ptr0, len0);
        }
        /**
        * @returns {(string)[] | undefined}
        */
        get assetPaths() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_sceneparameterobject_assetPaths(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getInt32Memory0()[retptr / 4 + 1];
                let v1;
                if (r0 !== 0) {
                    v1 = getArrayJsValueFromWasm0(r0, r1).slice();
                    wasm.__wbindgen_free(r0, r1 * 4);
                }
                return v1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        * @param {(string)[] | undefined} arg0
        */
        set assetPaths(arg0) {
            var ptr0 = isLikeNone(arg0) ? 0 : passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.__wbg_set_sceneparameterobject_assetPaths(this.__wbg_ptr, ptr0, len0);
        }
        /**
        * @returns {any[] | undefined}
        */
        get storageKeys() {
            try {
                const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
                wasm.__wbg_get_sceneparameterobject_storageKeys(retptr, this.__wbg_ptr);
                var r0 = getInt32Memory0()[retptr / 4 + 0];
                var r1 = getInt32Memory0()[retptr / 4 + 1];
                let v1;
                if (r0 !== 0) {
                    v1 = getArrayJsValueFromWasm0(r0, r1).slice();
                    wasm.__wbindgen_free(r0, r1 * 4);
                }
                return v1;
            } finally {
                wasm.__wbindgen_add_to_stack_pointer(16);
            }
        }
        /**
        * @param {any[] | undefined} arg0
        */
        set storageKeys(arg0) {
            var ptr0 = isLikeNone(arg0) ? 0 : passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.__wbg_set_sceneparameterobject_storageKeys(this.__wbg_ptr, ptr0, len0);
        }
        /**
        * @returns {boolean}
        */
        get local() {
            const ret = wasm.__wbg_get_sceneparameterobject_local(this.__wbg_ptr);
            return ret !== 0;
        }
        /**
        * @param {boolean} arg0
        */
        set local(arg0) {
            wasm.__wbg_set_sceneparameterobject_local(this.__wbg_ptr, arg0);
        }
        /**
        * @returns {string | undefined}
        */
        get name() {
            const ret = wasm.__wbg_get_sceneparameterobject_name(this.__wbg_ptr);
            return ret;
        }
        /**
        * @param {string | undefined} arg0
        */
        set name(arg0) {
            wasm.__wbg_set_sceneparameterobject_name(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addToExternrefTable0(arg0));
        }
    }
    __exports.SceneParameterObject = SceneParameterObject;

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    function __wbg_get_imports() {
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_new_922693aeee367f21 = function(arg0) {
            const ret = new g.FilledRect(FilledRectParameter.__wrap(arg0));
            return ret;
        };
        imports.wbg.__wbg_id_f261686d1760853c = function(arg0) {
            const ret = arg0.id;
            return ret;
        };
        imports.wbg.__wbg_x_302470bc7e5095b3 = function(arg0) {
            const ret = arg0.x;
            return ret;
        };
        imports.wbg.__wbg_y_38d3fedc06c277bd = function(arg0) {
            const ret = arg0.y;
            return ret;
        };
        imports.wbg.__wbg_onPointDown_6e3c1448c6cb4ffd = function(arg0) {
            const ret = arg0.onPointDown;
            return ret;
        };
        imports.wbg.__wbg_log_716d358914c44586 = function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbg_add_b263e1a567e2eda5 = function(arg0, arg1) {
            arg0.add(arg1);
        };
        imports.wbg.__wbg_modified_6f96956179fce28e = function(arg0) {
            arg0.modified();
        };
        imports.wbg.__wbg_id_12a1a14cc5946b59 = function(arg0) {
            const ret = arg0.id;
            return ret;
        };
        imports.wbg.__wbg_setx_b6feb9beb135d6fa = function(arg0, arg1) {
            arg0.x = arg1;
        };
        imports.wbg.__wbg_sety_49c2cdb1b53b349d = function(arg0, arg1) {
            arg0.y = arg1;
        };
        imports.wbg.__wbg_static_accessor_GAME_7c453f8c156b042d = function() {
            const ret = g.game;
            return ret;
        };
        imports.wbg.__wbg_scene_5dcb5555084efe6a = function(arg0) {
            const ret = arg0.scene();
            return ret;
        };
        imports.wbg.__wbg_pushScene_078bf604af6c8a5c = function(arg0, arg1, arg2) {
            const ret = arg0.pushScene(arg1, arg2);
            return ret;
        };
        imports.wbg.__wbg_point_f35cc5d2bb283e3d = function(arg0) {
            const ret = arg0.point;
            return ret;
        };
        imports.wbg.__wbg_x_417223d52e8f810e = function(arg0) {
            const ret = arg0.x;
            return ret;
        };
        imports.wbg.__wbg_y_398e9cccf1c984a4 = function(arg0) {
            const ret = arg0.y;
            return ret;
        };
        imports.wbg.__wbg_new_45d9a9569c7c114e = function(arg0) {
            const ret = new g.Scene(SceneParameterObject.__wrap(arg0));
            return ret;
        };
        imports.wbg.__wbg_append_a3de8b8a26e7e176 = function(arg0, arg1) {
            arg0.append(arg1);
        };
        imports.wbg.__wbg_onUpdate_5329fb00fa6f8f5e = function(arg0) {
            const ret = arg0.onUpdate;
            return ret;
        };
        imports.wbg.__wbg_onLoad_fd83476a09e5382b = function(arg0) {
            const ret = arg0.onLoad;
            return ret;
        };
        imports.wbg.__wbg_onPointDownCapture_1234399682b45f88 = function(arg0) {
            const ret = arg0.onPointDownCapture;
            return ret;
        };
        imports.wbg.__wbg_children_9c84bf48e70f7f04 = function(arg0, arg1) {
            const ret = arg1.children;
            const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len1;
            getInt32Memory0()[arg0 / 4 + 0] = ptr1;
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        };
        imports.wbg.__wbg_crypto_c48a774b022d20ac = function(arg0) {
            const ret = arg0.crypto;
            return ret;
        };
        imports.wbg.__wbg_msCrypto_bcb970640f50a1e8 = function(arg0) {
            const ret = arg0.msCrypto;
            return ret;
        };
        imports.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab = function() { return handleError(function (arg0, arg1) {
            arg0.getRandomValues(arg1);
        }, arguments) };
        imports.wbg.__wbg_randomFillSync_dc1e9a60c158336d = function() { return handleError(function (arg0, arg1) {
            arg0.randomFillSync(arg1);
        }, arguments) };
        imports.wbg.__wbg_require_8f08ceecec0f4fee = function() { return handleError(function () {
            const ret = module.require;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_process_298734cf255a885d = function(arg0) {
            const ret = arg0.process;
            return ret;
        };
        imports.wbg.__wbg_versions_e2e78e134e3e5d01 = function(arg0) {
            const ret = arg0.versions;
            return ret;
        };
        imports.wbg.__wbg_node_1cd7a5d853dbea79 = function(arg0) {
            const ret = arg0.node;
            return ret;
        };
        imports.wbg.__wbindgen_is_object = function(arg0) {
            const val = arg0;
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        };
        imports.wbg.__wbindgen_is_string = function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        };
        imports.wbg.__wbg_newnoargs_581967eacc0e2604 = function(arg0, arg1) {
            const ret = new Function(getStringFromWasm0(arg0, arg1));
            return ret;
        };
        imports.wbg.__wbg_call_cb65541d95d71282 = function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_call_01734de55d61e11d = function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments) };
        imports.wbg.__wbg_globalThis_1d39714405582d3c = function() { return handleError(function () {
            const ret = globalThis.globalThis;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_self_1ff1d729e9aae938 = function() { return handleError(function () {
            const ret = self.self;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_window_5f4faef6c12b79ec = function() { return handleError(function () {
            const ret = window.window;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_global_651f05c6a0944d1c = function() { return handleError(function () {
            const ret = global.global;
            return ret;
        }, arguments) };
        imports.wbg.__wbg_new_8125e318e6245eed = function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        };
        imports.wbg.__wbg_newwithlength_e5d69174d6984cd7 = function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa = function(arg0, arg1, arg2) {
            const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_subarray_13db269f57aa838d = function(arg0, arg1, arg2) {
            const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
            return ret;
        };
        imports.wbg.__wbg_set_5cf90238115182c3 = function(arg0, arg1, arg2) {
            arg0.set(arg1, arg2 >>> 0);
        };
        imports.wbg.__wbindgen_is_function = function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        };
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        };
        imports.wbg.__wbg_buffer_085ec1f694018c4f = function(arg0) {
            const ret = arg0.buffer;
            return ret;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_memory = function() {
            const ret = wasm.memory;
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper1195 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 362, __wbg_adapter_22);
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper1197 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 360, __wbg_adapter_25);
            return ret;
        };
        imports.wbg.__wbindgen_closure_wrapper3648 = function(arg0, arg1, arg2) {
            const ret = makeMutClosure(arg0, arg1, 705, __wbg_adapter_28);
            return ret;
        };
        imports.wbg.__wbindgen_init_externref_table = function() {
            const table = wasm.__wbindgen_export_0;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
            ;
        };

        return imports;
    }

    function __wbg_init_memory(imports, maybe_memory) {

    }

    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        __wbg_init.__wbindgen_wasm_module = module;
        cachedFloat32Memory0 = null;
        cachedInt32Memory0 = null;
        cachedUint32Memory0 = null;
        cachedUint8Memory0 = null;

        wasm.__wbindgen_start();
        return wasm;
    }

    function initSync(module) {
        if (wasm !== undefined) return wasm;

        const imports = __wbg_get_imports();

        __wbg_init_memory(imports);

        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }

        const instance = new WebAssembly.Instance(module, imports);

        return __wbg_finalize_init(instance, module);
    }

    async function __wbg_init(input) {
        if (wasm !== undefined) return wasm;

        if (typeof input === 'undefined' && script_src !== 'undefined') {
            input = script_src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = __wbg_get_imports();

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        __wbg_init_memory(imports);

        const { instance, module } = await __wbg_load(await input, imports);

        return __wbg_finalize_init(instance, module);
    }

    wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);module.exports = {wasm_bindgen,  locateFile: path => g.game._assetManager.configuration.wasm.path};

})();
