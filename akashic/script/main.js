
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
        }
        
        g.feedLabelProperties = (entity, text, textAlign, textColor, widthAutoAdjust) => {
            entity.text = text
            console.log(entity.text)
            entity.textAlign = textAlign
            entity.textColor = textColor
            entity.widthAutoAdjust = widthAutoAdjust
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
            let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src = "";
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm = undefined;

    const heap = new Array(128).fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

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

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
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
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_44(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7de1fbbd9d5003a9(arg0, arg1);
}

function __wbg_adapter_47(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0c5f1aa0ca71cf69(arg0, arg1, addHeapObject(arg2));
}

let cachedFloat32Memory0 = null;

function getFloat32Memory0() {
    if (cachedFloat32Memory0 === null || cachedFloat32Memory0.byteLength === 0) {
        cachedFloat32Memory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32Memory0;
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
        result.push(takeObject(slice[i]));
    }
    return result;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
class AkashicEntityParam {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_akashicentityparam_free(ptr);
    }
    /**
    * @returns {number | undefined}
    */
    get x() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_x(retptr, this.__wbg_ptr);
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
    set x(arg0) {
        wasm.__wbg_set_akashicentityparam_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get y() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_y(retptr, this.__wbg_ptr);
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
    set y(arg0) {
        wasm.__wbg_set_akashicentityparam_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get width() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_width(retptr, this.__wbg_ptr);
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
    set width(arg0) {
        wasm.__wbg_set_akashicentityparam_width(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get height() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_height(retptr, this.__wbg_ptr);
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
    set height(arg0) {
        wasm.__wbg_set_akashicentityparam_height(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get opacity() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_opacity(retptr, this.__wbg_ptr);
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
    set opacity(arg0) {
        wasm.__wbg_set_akashicentityparam_opacity(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_scaleX(retptr, this.__wbg_ptr);
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
    set scaleX(arg0) {
        wasm.__wbg_set_akashicentityparam_scaleX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_scaleY(retptr, this.__wbg_ptr);
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
    set scaleY(arg0) {
        wasm.__wbg_set_akashicentityparam_scaleY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get angle() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_angle(retptr, this.__wbg_ptr);
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
    set angle(arg0) {
        wasm.__wbg_set_akashicentityparam_angle(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get compositeOperation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_compositeOperation(retptr, this.__wbg_ptr);
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
    set compositeOperation(arg0) {
        wasm.__wbg_set_akashicentityparam_compositeOperation(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_anchorX(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_akashicentityparam_anchorX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_anchorY(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_akashicentityparam_anchorY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get scene() {
        const ret = wasm.__wbg_get_akashicentityparam_scene(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set scene(arg0) {
        wasm.__wbg_set_akashicentityparam_scene(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {boolean}
    */
    get local() {
        const ret = wasm.__wbg_get_akashicentityparam_local(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set local(arg0) {
        wasm.__wbg_set_akashicentityparam_local(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {any}
    */
    get parent() {
        const ret = wasm.__wbg_get_akashicentityparam_parent(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set parent(arg0) {
        wasm.__wbg_set_akashicentityparam_parent(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any[]}
    */
    get children() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_children(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {any[]} arg0
    */
    set children(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_akashicentityparam_children(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {boolean}
    */
    get touchable() {
        const ret = wasm.__wbg_get_akashicentityparam_touchable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set touchable(arg0) {
        wasm.__wbg_set_akashicentityparam_touchable(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_akashicentityparam_id(retptr, this.__wbg_ptr);
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
    set id(arg0) {
        wasm.__wbg_set_akashicentityparam_id(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get tag() {
        const ret = wasm.__wbg_get_akashicentityparam_tag(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set tag(arg0) {
        wasm.__wbg_set_akashicentityparam_tag(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any | undefined}
    */
    get shaderProgram() {
        const ret = wasm.__wbg_get_akashicentityparam_shaderProgram(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any | undefined} arg0
    */
    set shaderProgram(arg0) {
        wasm.__wbg_set_akashicentityparam_shaderProgram(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.AkashicEntityParam = AkashicEntityParam;
/**
*/
class BitmapFontParam {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bitmapfontparam_free(ptr);
    }
    /**
    * @returns {any}
    */
    get src() {
        const ret = wasm.__wbg_get_bitmapfontparam_src(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set src(arg0) {
        wasm.__wbg_set_bitmapfontparam_src(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {number | undefined}
    */
    get defaultGlyphHeight() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_bitmapfontparam_defaultGlyphHeight(retptr, this.__wbg_ptr);
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
    set defaultGlyphHeight(arg0) {
        wasm.__wbg_set_bitmapfontparam_defaultGlyphHeight(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get defaultGlyphWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_bitmapfontparam_defaultGlyphWidth(retptr, this.__wbg_ptr);
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
    set defaultGlyphWidth(arg0) {
        wasm.__wbg_set_bitmapfontparam_defaultGlyphWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get map() {
        const ret = wasm.__wbg_get_bitmapfontparam_map(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set map(arg0) {
        wasm.__wbg_set_bitmapfontparam_map(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any}
    */
    get glyphInfo() {
        const ret = wasm.__wbg_get_bitmapfontparam_glyphInfo(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set glyphInfo(arg0) {
        wasm.__wbg_set_bitmapfontparam_glyphInfo(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {GlyphArea | undefined}
    */
    get missingGlyph() {
        const ret = wasm.__wbg_get_bitmapfontparam_missingGlyph(this.__wbg_ptr);
        return ret === 0 ? undefined : GlyphArea.__wrap(ret);
    }
    /**
    * @param {GlyphArea | undefined} arg0
    */
    set missingGlyph(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, GlyphArea);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_bitmapfontparam_missingGlyph(this.__wbg_ptr, ptr0);
    }
}
__exports.BitmapFontParam = BitmapFontParam;
/**
*/
class DynamicFontHint {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DynamicFontHint.prototype);
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
        wasm.__wbg_dynamicfonthint_free(ptr);
    }
    /**
    * @returns {number | undefined}
    */
    get initialAtlasWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_initialAtlasWidth(retptr, this.__wbg_ptr);
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
    set initialAtlasWidth(arg0) {
        wasm.__wbg_set_dynamicfonthint_initialAtlasWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get initialAtlasHeight() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_initialAtlasHeight(retptr, this.__wbg_ptr);
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
    set initialAtlasHeight(arg0) {
        wasm.__wbg_set_dynamicfonthint_initialAtlasHeight(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get maxAtlasWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_maxAtlasWidth(retptr, this.__wbg_ptr);
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
    set maxAtlasWidth(arg0) {
        wasm.__wbg_set_dynamicfonthint_maxAtlasWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get maxAtlasHeight() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_maxAtlasHeight(retptr, this.__wbg_ptr);
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
    set maxAtlasHeight(arg0) {
        wasm.__wbg_set_dynamicfonthint_maxAtlasHeight(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get maxAtlasNum() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_maxAtlasNum(retptr, this.__wbg_ptr);
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
    set maxAtlasNum(arg0) {
        wasm.__wbg_set_dynamicfonthint_maxAtlasNum(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
}
__exports.DynamicFontHint = DynamicFontHint;
/**
*/
class DynamicFontParam {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dynamicfontparam_free(ptr);
    }
    /**
    * @returns {any}
    */
    get game() {
        const ret = wasm.__wbg_get_dynamicfontparam_game(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set game(arg0) {
        wasm.__wbg_set_dynamicfontparam_game(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {string | string[]}
    */
    get fontFamily() {
        const ret = wasm.__wbg_get_dynamicfontparam_fontFamily(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string | string[]} arg0
    */
    set fontFamily(arg0) {
        wasm.__wbg_set_dynamicfontparam_fontFamily(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {number}
    */
    get size() {
        const ret = wasm.__wbg_get_dynamicfontparam_size(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set size(arg0) {
        wasm.__wbg_set_dynamicfontparam_size(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {DynamicFontHint | undefined}
    */
    get hint() {
        const ret = wasm.__wbg_get_dynamicfontparam_hint(this.__wbg_ptr);
        return ret === 0 ? undefined : DynamicFontHint.__wrap(ret);
    }
    /**
    * @param {DynamicFontHint | undefined} arg0
    */
    set hint(arg0) {
        let ptr0 = 0;
        if (!isLikeNone(arg0)) {
            _assertClass(arg0, DynamicFontHint);
            ptr0 = arg0.__destroy_into_raw();
        }
        wasm.__wbg_set_dynamicfontparam_hint(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string | undefined}
    */
    get fontColor() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfontparam_fontColor(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} arg0
    */
    set fontColor(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_dynamicfontparam_fontColor(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {'normal' | 'bold' | undefined}
    */
    get fontWeight() {
        const ret = wasm.__wbg_get_dynamicfontparam_fontWeight(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {'normal' | 'bold' | undefined} arg0
    */
    set fontWeight(arg0) {
        wasm.__wbg_set_dynamicfontparam_fontWeight(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
    /**
    * @returns {number | undefined}
    */
    get strokeWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfonthint_initialAtlasHeight(retptr, this.__wbg_ptr);
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
    set strokeWidth(arg0) {
        wasm.__wbg_set_dynamicfonthint_initialAtlasHeight(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {string | undefined}
    */
    get strokeColor() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_dynamicfontparam_strokeColor(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} arg0
    */
    set strokeColor(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_dynamicfontparam_strokeColor(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {boolean | undefined}
    */
    get strokeOnly() {
        const ret = wasm.__wbg_get_dynamicfontparam_strokeOnly(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * @param {boolean | undefined} arg0
    */
    set strokeOnly(arg0) {
        wasm.__wbg_set_dynamicfontparam_strokeOnly(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
    /**
    * @returns {any | undefined}
    */
    get surfaceAtlasSet() {
        const ret = wasm.__wbg_get_dynamicfontparam_surfaceAtlasSet(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any | undefined} arg0
    */
    set surfaceAtlasSet(arg0) {
        wasm.__wbg_set_dynamicfontparam_surfaceAtlasSet(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.DynamicFontParam = DynamicFontParam;
/**
*/
class EntityProperties {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_entityproperties_free(ptr);
    }
    /**
    * @returns {number}
    */
    get id() {
        const ret = wasm.__wbg_get_entityproperties_id(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set id(arg0) {
        wasm.__wbg_set_entityproperties_id(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x() {
        const ret = wasm.__wbg_get_entityproperties_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_entityproperties_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        const ret = wasm.__wbg_get_entityproperties_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_entityproperties_y(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get width() {
        const ret = wasm.__wbg_get_entityproperties_width(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_entityproperties_width(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get height() {
        const ret = wasm.__wbg_get_entityproperties_height(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_entityproperties_height(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get angle() {
        const ret = wasm.__wbg_get_entityproperties_angle(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set angle(arg0) {
        wasm.__wbg_set_entityproperties_angle(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get scale_x() {
        const ret = wasm.__wbg_get_entityproperties_scale_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set scale_x(arg0) {
        wasm.__wbg_set_entityproperties_scale_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get scale_y() {
        const ret = wasm.__wbg_get_entityproperties_scale_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set scale_y(arg0) {
        wasm.__wbg_set_entityproperties_scale_y(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchor_x() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_entityproperties_anchor_x(retptr, this.__wbg_ptr);
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
    set anchor_x(arg0) {
        wasm.__wbg_set_entityproperties_anchor_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchor_y() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_entityproperties_anchor_y(retptr, this.__wbg_ptr);
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
    set anchor_y(arg0) {
        wasm.__wbg_set_entityproperties_anchor_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {boolean}
    */
    get touchable() {
        const ret = wasm.__wbg_get_entityproperties_touchable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set touchable(arg0) {
        wasm.__wbg_set_entityproperties_touchable(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get visible() {
        const ret = wasm.__wbg_get_entityproperties_visible(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set visible(arg0) {
        wasm.__wbg_set_entityproperties_visible(this.__wbg_ptr, arg0);
    }
}
__exports.EntityProperties = EntityProperties;
/**
*/
class FilledRectParam {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FilledRectParam.prototype);
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
        wasm.__wbg_filledrectparam_free(ptr);
    }
    /**
    * @returns {string}
    */
    get cssColor() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_cssColor(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_filledrectparam_cssColor(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {number}
    */
    get width() {
        const ret = wasm.__wbg_get_filledrectparam_width(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        wasm.__wbg_set_filledrectparam_width(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get height() {
        const ret = wasm.__wbg_get_filledrectparam_height(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_filledrectparam_height(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get x() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_x(retptr, this.__wbg_ptr);
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
    set x(arg0) {
        wasm.__wbg_set_filledrectparam_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get y() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_y(retptr, this.__wbg_ptr);
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
    set y(arg0) {
        wasm.__wbg_set_filledrectparam_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get opacity() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_opacity(retptr, this.__wbg_ptr);
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
    set opacity(arg0) {
        wasm.__wbg_set_filledrectparam_opacity(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_scaleX(retptr, this.__wbg_ptr);
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
    set scaleX(arg0) {
        wasm.__wbg_set_filledrectparam_scaleX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_scaleY(retptr, this.__wbg_ptr);
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
    set scaleY(arg0) {
        wasm.__wbg_set_filledrectparam_scaleY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get angle() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_angle(retptr, this.__wbg_ptr);
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
    set angle(arg0) {
        wasm.__wbg_set_filledrectparam_angle(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get compositeOperation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_compositeOperation(retptr, this.__wbg_ptr);
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
    set compositeOperation(arg0) {
        wasm.__wbg_set_filledrectparam_compositeOperation(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_anchorX(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_filledrectparam_anchorX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_anchorY(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_filledrectparam_anchorY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get scene() {
        const ret = wasm.__wbg_get_filledrectparam_scene(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set scene(arg0) {
        wasm.__wbg_set_filledrectparam_scene(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {boolean}
    */
    get local() {
        const ret = wasm.__wbg_get_filledrectparam_local(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set local(arg0) {
        wasm.__wbg_set_filledrectparam_local(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {any}
    */
    get parent() {
        const ret = wasm.__wbg_get_filledrectparam_parent(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set parent(arg0) {
        wasm.__wbg_set_filledrectparam_parent(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any[]}
    */
    get children() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_children(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {any[]} arg0
    */
    set children(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_filledrectparam_children(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {boolean}
    */
    get touchable() {
        const ret = wasm.__wbg_get_filledrectparam_touchable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set touchable(arg0) {
        wasm.__wbg_set_filledrectparam_touchable(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_filledrectparam_id(retptr, this.__wbg_ptr);
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
    set id(arg0) {
        wasm.__wbg_set_filledrectparam_id(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get tag() {
        const ret = wasm.__wbg_get_filledrectparam_tag(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set tag(arg0) {
        wasm.__wbg_set_filledrectparam_tag(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any | undefined}
    */
    get shaderProgram() {
        const ret = wasm.__wbg_get_filledrectparam_shaderProgram(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any | undefined} arg0
    */
    set shaderProgram(arg0) {
        wasm.__wbg_set_filledrectparam_shaderProgram(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.FilledRectParam = FilledRectParam;
/**
*/
class GlyphArea {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GlyphArea.prototype);
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
        wasm.__wbg_glypharea_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        const ret = wasm.__wbg_get_glypharea_x(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_glypharea_x(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        const ret = wasm.__wbg_get_glypharea_y(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_glypharea_y(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get height() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_glypharea_height(retptr, this.__wbg_ptr);
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
    set height(arg0) {
        wasm.__wbg_set_glypharea_height(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get width() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_glypharea_width(retptr, this.__wbg_ptr);
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
    set width(arg0) {
        wasm.__wbg_set_glypharea_width(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get offsetX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_glypharea_offsetX(retptr, this.__wbg_ptr);
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
    set offsetX(arg0) {
        wasm.__wbg_set_glypharea_offsetX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get offsetY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_glypharea_offsetY(retptr, this.__wbg_ptr);
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
    set offsetY(arg0) {
        wasm.__wbg_set_glypharea_offsetY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get advanceWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_glypharea_advanceWidth(retptr, this.__wbg_ptr);
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
    set advanceWidth(arg0) {
        wasm.__wbg_set_glypharea_advanceWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
}
__exports.GlyphArea = GlyphArea;
/**
*/
class LabelParam {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_labelparam_free(ptr);
    }
    /**
    * @returns {string}
    */
    get text() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_text(retptr, this.__wbg_ptr);
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
    set text(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labelparam_text(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {any}
    */
    get font() {
        const ret = wasm.__wbg_get_labelparam_font(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set font(arg0) {
        wasm.__wbg_set_labelparam_font(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {number | undefined}
    */
    get fontSize() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_fontSize(retptr, this.__wbg_ptr);
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
    set fontSize(arg0) {
        wasm.__wbg_set_labelparam_fontSize(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {string | undefined}
    */
    get textAlign() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_textAlign(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} arg0
    */
    set textAlign(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labelparam_textAlign(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {number | undefined}
    */
    get maxWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_maxWidth(retptr, this.__wbg_ptr);
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
    set maxWidth(arg0) {
        wasm.__wbg_set_labelparam_maxWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {boolean | undefined}
    */
    get widthAutoAdjust() {
        const ret = wasm.__wbg_get_labelparam_widthAutoAdjust(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * @param {boolean | undefined} arg0
    */
    set widthAutoAdjust(arg0) {
        wasm.__wbg_set_labelparam_widthAutoAdjust(this.__wbg_ptr, isLikeNone(arg0) ? 0xFFFFFF : arg0 ? 1 : 0);
    }
    /**
    * @returns {string | undefined}
    */
    get textColor() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_textColor(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} arg0
    */
    set textColor(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labelparam_textColor(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {number | undefined}
    */
    get x() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_x(retptr, this.__wbg_ptr);
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
    set x(arg0) {
        wasm.__wbg_set_labelparam_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get y() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_y(retptr, this.__wbg_ptr);
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
    set y(arg0) {
        wasm.__wbg_set_labelparam_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get width() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_width(retptr, this.__wbg_ptr);
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
    set width(arg0) {
        wasm.__wbg_set_labelparam_width(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get height() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_height(retptr, this.__wbg_ptr);
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
    set height(arg0) {
        wasm.__wbg_set_labelparam_height(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get opacity() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_opacity(retptr, this.__wbg_ptr);
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
    set opacity(arg0) {
        wasm.__wbg_set_labelparam_opacity(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_scaleX(retptr, this.__wbg_ptr);
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
    set scaleX(arg0) {
        wasm.__wbg_set_labelparam_scaleX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_scaleY(retptr, this.__wbg_ptr);
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
    set scaleY(arg0) {
        wasm.__wbg_set_labelparam_scaleY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get angle() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_angle(retptr, this.__wbg_ptr);
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
    set angle(arg0) {
        wasm.__wbg_set_labelparam_angle(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get compositeOperation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_compositeOperation(retptr, this.__wbg_ptr);
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
    set compositeOperation(arg0) {
        wasm.__wbg_set_labelparam_compositeOperation(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_anchorX(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_labelparam_anchorX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_anchorY(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_labelparam_anchorY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get scene() {
        const ret = wasm.__wbg_get_labelparam_scene(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set scene(arg0) {
        wasm.__wbg_set_labelparam_scene(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {boolean}
    */
    get local() {
        const ret = wasm.__wbg_get_labelparam_local(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set local(arg0) {
        wasm.__wbg_set_labelparam_local(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {any}
    */
    get parent() {
        const ret = wasm.__wbg_get_labelparam_parent(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set parent(arg0) {
        wasm.__wbg_set_labelparam_parent(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any[]}
    */
    get children() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_children(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {any[]} arg0
    */
    set children(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_labelparam_children(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {boolean}
    */
    get touchable() {
        const ret = wasm.__wbg_get_labelparam_touchable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set touchable(arg0) {
        wasm.__wbg_set_labelparam_touchable(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_labelparam_id(retptr, this.__wbg_ptr);
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
    set id(arg0) {
        wasm.__wbg_set_labelparam_id(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get tag() {
        const ret = wasm.__wbg_get_labelparam_tag(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set tag(arg0) {
        wasm.__wbg_set_labelparam_tag(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any | undefined}
    */
    get shaderProgram() {
        const ret = wasm.__wbg_get_labelparam_shaderProgram(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any | undefined} arg0
    */
    set shaderProgram(arg0) {
        wasm.__wbg_set_labelparam_shaderProgram(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.LabelParam = LabelParam;
/**
*/
class SceneParameterObject {

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
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set game(arg0) {
        wasm.__wbg_set_sceneparameterobject_game(this.__wbg_ptr, addHeapObject(arg0));
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
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} arg0
    */
    set name(arg0) {
        wasm.__wbg_set_sceneparameterobject_name(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.SceneParameterObject = SceneParameterObject;
/**
*/
class SpriteParam {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_spriteparam_free(ptr);
    }
    /**
    * @returns {any}
    */
    get src() {
        const ret = wasm.__wbg_get_spriteparam_src(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set src(arg0) {
        wasm.__wbg_set_spriteparam_src(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {number | undefined}
    */
    get srcWidth() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_srcWidth(retptr, this.__wbg_ptr);
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
    set srcWidth(arg0) {
        wasm.__wbg_set_spriteparam_srcWidth(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get srcHeight() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_srcHeight(retptr, this.__wbg_ptr);
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
    set srcHeight(arg0) {
        wasm.__wbg_set_spriteparam_srcHeight(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get srcX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_srcX(retptr, this.__wbg_ptr);
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
    set srcX(arg0) {
        wasm.__wbg_set_spriteparam_srcX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get srcY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_srcY(retptr, this.__wbg_ptr);
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
    set srcY(arg0) {
        wasm.__wbg_set_spriteparam_srcY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get x() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_x(retptr, this.__wbg_ptr);
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
    set x(arg0) {
        wasm.__wbg_set_spriteparam_x(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get y() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_y(retptr, this.__wbg_ptr);
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
    set y(arg0) {
        wasm.__wbg_set_spriteparam_y(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get width() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_width(retptr, this.__wbg_ptr);
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
    set width(arg0) {
        wasm.__wbg_set_spriteparam_width(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get height() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_height(retptr, this.__wbg_ptr);
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
    set height(arg0) {
        wasm.__wbg_set_spriteparam_height(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get opacity() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_opacity(retptr, this.__wbg_ptr);
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
    set opacity(arg0) {
        wasm.__wbg_set_spriteparam_opacity(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_scaleX(retptr, this.__wbg_ptr);
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
    set scaleX(arg0) {
        wasm.__wbg_set_spriteparam_scaleX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get scaleY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_scaleY(retptr, this.__wbg_ptr);
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
    set scaleY(arg0) {
        wasm.__wbg_set_spriteparam_scaleY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get angle() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_angle(retptr, this.__wbg_ptr);
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
    set angle(arg0) {
        wasm.__wbg_set_spriteparam_angle(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get compositeOperation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_compositeOperation(retptr, this.__wbg_ptr);
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
    set compositeOperation(arg0) {
        wasm.__wbg_set_spriteparam_compositeOperation(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorX() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_anchorX(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_spriteparam_anchorX(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get anchorY() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_anchorY(retptr, this.__wbg_ptr);
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
        wasm.__wbg_set_spriteparam_anchorY(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get scene() {
        const ret = wasm.__wbg_get_spriteparam_scene(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set scene(arg0) {
        wasm.__wbg_set_spriteparam_scene(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {boolean}
    */
    get local() {
        const ret = wasm.__wbg_get_spriteparam_local(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set local(arg0) {
        wasm.__wbg_set_spriteparam_local(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {any}
    */
    get parent() {
        const ret = wasm.__wbg_get_spriteparam_parent(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set parent(arg0) {
        wasm.__wbg_set_spriteparam_parent(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any[]}
    */
    get children() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_children(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {any[]} arg0
    */
    set children(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_spriteparam_children(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {boolean}
    */
    get touchable() {
        const ret = wasm.__wbg_get_spriteparam_touchable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set touchable(arg0) {
        wasm.__wbg_set_spriteparam_touchable(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_spriteparam_id(retptr, this.__wbg_ptr);
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
    set id(arg0) {
        wasm.__wbg_set_spriteparam_id(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {any}
    */
    get tag() {
        const ret = wasm.__wbg_get_spriteparam_tag(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set tag(arg0) {
        wasm.__wbg_set_spriteparam_tag(this.__wbg_ptr, addHeapObject(arg0));
    }
    /**
    * @returns {any | undefined}
    */
    get shaderProgram() {
        const ret = wasm.__wbg_get_spriteparam_shaderProgram(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {any | undefined} arg0
    */
    set shaderProgram(arg0) {
        wasm.__wbg_set_spriteparam_shaderProgram(this.__wbg_ptr, isLikeNone(arg0) ? 0 : addHeapObject(arg0));
    }
}
__exports.SpriteParam = SpriteParam;

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
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getEntityProperties_c8edc2df30c9e47c = function(arg0) {
        const ret = g.getEntityProperties(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_feedLabelProperties_bc8f9eaf82d0c9c8 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        let deferred0_0;
        let deferred0_1;
        let deferred1_0;
        let deferred1_1;
        try {
            deferred0_0 = arg1;
            deferred0_1 = arg2;
            deferred1_0 = arg3;
            deferred1_1 = arg4;
            let v2;
            if (arg5 !== 0) {
                v2 = getStringFromWasm0(arg5, arg6).slice();
                wasm.__wbindgen_free(arg5, arg6 * 1);
            }
            g.feedLabelProperties(getObject(arg0), getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), v2, arg7 !== 0);
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    };
    imports.wbg.__wbg_feedFilledRectProperties_9a19584032119101 = function(arg0, arg1, arg2) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg1;
            deferred0_1 = arg2;
            g.feedFilledRectProperties(getObject(arg0), getStringFromWasm0(arg1, arg2));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_feedEntityProperties_a74a6e89ce13e79e = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) {
        g.feedEntityProperties(getObject(arg0), arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8 === 0 ? undefined : arg9, arg10 === 0 ? undefined : arg11, arg12 !== 0, arg13 !== 0);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_invalidate_9ccfc734efa3f274 = function(arg0) {
        getObject(arg0).invalidate();
    };
    imports.wbg.__wbg_scene_5dcb5555084efe6a = function(arg0) {
        const ret = getObject(arg0).scene();
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_id_6ca5403de5216c84 = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_player_4036e913811318d0 = function(arg0) {
        const ret = getObject(arg0).player;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_id_9b2310e9bec069bc = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_id_f0f0b3b4e76a222a = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_getAllImages_5a7935ba8524b8f4 = function(arg0, arg1, arg2, arg3) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg2;
            deferred0_1 = arg3;
            const ret = getObject(arg1).getAllImages(getStringFromWasm0(arg2, arg3));
            const ptr2 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len2;
            getInt32Memory0()[arg0 / 4 + 0] = ptr2;
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getAllAudios_6082ce338217d45a = function(arg0, arg1, arg2, arg3) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg2;
            deferred0_1 = arg3;
            const ret = getObject(arg1).getAllAudios(getStringFromWasm0(arg2, arg3));
            const ptr2 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len2;
            getInt32Memory0()[arg0 / 4 + 0] = ptr2;
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getAllTexts_e4b38a0cdffee4ee = function(arg0, arg1, arg2, arg3) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg2;
            deferred0_1 = arg3;
            const ret = getObject(arg1).getAllTexts(getStringFromWasm0(arg2, arg3));
            const ptr2 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
            const len2 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len2;
            getInt32Memory0()[arg0 / 4 + 0] = ptr2;
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_gameState_33429d8df49b6749 = function(arg0) {
        const ret = getObject(arg0).gameState;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_setgameState_4b59dc3bed6d322c = function(arg0, arg1) {
        getObject(arg0).gameState = takeObject(arg1);
    };
    imports.wbg.__wbg_add_b263e1a567e2eda5 = function(arg0, arg1) {
        getObject(arg0).add(takeObject(arg1));
    };
    imports.wbg.__wbg_target_2bd19dc409d59bd4 = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_target_5c48ba6f763ac240 = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_new_922693aeee367f21 = function(arg0) {
        const ret = new g.FilledRect(FilledRectParam.__wrap(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_cssColor_6af5b452497db75b = function(arg0, arg1) {
        const ret = getObject(arg1).cssColor;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_id_43b31a7ccc967443 = function(arg0) {
        const ret = getObject(arg0).id;
        return ret;
    };
    imports.wbg.__wbg_append_a49587659876cef5 = function(arg0, arg1) {
        getObject(arg0).append(takeObject(arg1));
    };
    imports.wbg.__wbg_destroy_951d5729925dd7aa = function(arg0) {
        getObject(arg0).destroy();
    };
    imports.wbg.__wbg_modified_b013564fb16080e9 = function(arg0) {
        getObject(arg0).modified();
    };
    imports.wbg.__wbg_static_accessor_GAME_7c453f8c156b042d = function() {
        const ret = g.game;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_append_a3de8b8a26e7e176 = function(arg0, arg1) {
        getObject(arg0).append(takeObject(arg1));
    };
    imports.wbg.__wbg_onUpdate_5329fb00fa6f8f5e = function(arg0) {
        const ret = getObject(arg0).onUpdate;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_asset_a45570ff7d459c60 = function(arg0) {
        const ret = getObject(arg0).asset;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_onPointDownCapture_1234399682b45f88 = function(arg0) {
        const ret = getObject(arg0).onPointDownCapture;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_onPointUpCapture_9c8ce8c1073d9dbf = function(arg0) {
        const ret = getObject(arg0).onPointUpCapture;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_onPointMoveCapture_192ace04ec65390c = function(arg0) {
        const ret = getObject(arg0).onPointMoveCapture;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_target_51a4a962b9d019bc = function(arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_width_1115571ad95b8ed7 = function(arg0) {
        const ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_height_9755bceac268a7aa = function(arg0) {
        const ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_random_9f4c9b6499056ff4 = function(arg0) {
        const ret = getObject(arg0).random;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_localRandom_a70002f5f69dc224 = function(arg0) {
        const ret = getObject(arg0).localRandom;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_selfId_f7ef147fc162bc09 = function(arg0, arg1) {
        const ret = getObject(arg1).selfId;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_vars_5c5e9a6d2447d420 = function(arg0) {
        const ret = getObject(arg0).vars;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_onJoin_c52ab3146286ccf8 = function(arg0) {
        const ret = getObject(arg0).onJoin;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_id_bba5ab5374c68557 = function(arg0, arg1) {
        const ret = getObject(arg1).id;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_getwithrefkey_d1f0d12f1f1b63ea = function(arg0, arg1) {
        const ret = getObject(arg0)[getObject(arg1)];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_now_0cfdc90c97d0c24b = function(arg0) {
        const ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__wbg_crypto_c48a774b022d20ac = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_process_298734cf255a885d = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_e2e78e134e3e5d01 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_1cd7a5d853dbea79 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_msCrypto_bcb970640f50a1e8 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_8f08ceecec0f4fee = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_dc1e9a60c158336d = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_newnoargs_581967eacc0e2604 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_97b561fb56f034b5 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_cb65541d95d71282 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_b51585de1b234aff = function() {
        const ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_1ff1d729e9aae938 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_5f4faef6c12b79ec = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_1d39714405582d3c = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_651f05c6a0944d1c = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_ArrayBuffer_39ac22089b74fddb = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_call_01734de55d61e11d = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_bb8e18dd21c97288 = function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_buffer_085ec1f694018c4f = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_8125e318e6245eed = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_5cf90238115182c3 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_72e2208bbc0efc61 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Uint8Array_d8d9cb2b8e8ac1d4 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_e5d69174d6984cd7 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_13db269f57aa838d = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_defineProperty_301e4e0c0089acd1 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.defineProperty(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_092e06b0f9d71865 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
        const v = getObject(arg1);
        const ret = typeof(v) === 'bigint' ? v : undefined;
        getBigInt64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? BigInt(0) : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1899 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1031, __wbg_adapter_44);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1901 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1031, __wbg_adapter_47);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1903 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1031, __wbg_adapter_47);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1905 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1031, __wbg_adapter_47);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1907 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 1031, __wbg_adapter_47);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedBigInt64Memory0 = null;
    cachedFloat32Memory0 = null;
    cachedFloat64Memory0 = null;
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
        
        if (typeof window == 'undefined') {
            input = fetch(g.game._assetManager.configuration.wasm.path)
        }else{
            input = fetch(g.game._assetManager.configuration.wasm.path)
        }
    
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports)();

})();

        })
        
        g.game.pushScene(scene)
    }
