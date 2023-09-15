use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

pub mod entity;

#[inline]
pub fn expand_object_2d(input: TokenStream) -> TokenStream {
    try_expand_object_2d(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


pub(crate) fn try_expand_object_2d(input: TokenStream) -> syn::Result<TokenStream2> {
    let entity = syn::parse::<ItemStruct>(input)?;
    let entity_name = &entity.ident;

    Ok(quote! {
        #[wasm_bindgen(js_namespace=g)]
        extern "C" {
            #[wasm_bindgen(method, getter, js_name=width)]
            fn _width(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=width)]
            fn _set_width(this: &#entity_name, width: f32);

            #[wasm_bindgen(method, getter, js_name=height)]
            fn _height(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=height)]
            fn _set_height(this: &#entity_name, height: f32);

            #[wasm_bindgen(method, getter, js_name=x)]
            fn _x(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=x)]
            fn _set_x(this: &#entity_name, x: f32);

            #[wasm_bindgen(method, getter, js_name=y)]
            fn _y(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=y)]
            fn _set_y(this: &#entity_name, y: f32);

            #[wasm_bindgen(method, getter, js_name=opacity)]
            fn _opacity(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=opacity)]
            fn _set_opacity(this: &#entity_name, opacity: f32);

            #[wasm_bindgen(method, getter, js_name=anchorX)]
            fn _anchor_x(this: &#entity_name) -> Option<f32>;

            #[wasm_bindgen(method, setter, js_name=anchorX)]
            fn _set_anchor_x(this: &#entity_name, anchor_x: Option<f32>);

            #[wasm_bindgen(method, getter, js_name=anchorY)]
            fn _anchor_y(this: &#entity_name) -> Option<f32>;

            #[wasm_bindgen(method, setter, js_name=anchorY)]
            fn _set_anchor_y(this: &#entity_name, anchor_y: Option<f32>);

            #[wasm_bindgen(method, js_name=anchor)]
            fn _anchor(this: &#entity_name, x: f32, y: f32);

            #[wasm_bindgen(method, js_name=moveBy)]
            fn _move_by(this: &#entity_name, x: f32, y: f32);

            #[wasm_bindgen(method, js_name=moveTo)]
            fn _move_to(this: &#entity_name, x: f32, y: f32);

            #[wasm_bindgen(method, js_name=resizeBy)]
            fn _resize_by(this: &#entity_name, width: f32, height: f32);

            #[wasm_bindgen(method, js_name=resizeTo)]
            fn _resize_to(this: &#entity_name, width: f32, height: f32);

            #[wasm_bindgen(method, js_name=scale)]
            fn _scale(this: &#entity_name, scale: f32);

            #[wasm_bindgen(method, getter, js_name = angle)]
            fn _angle(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=angle)]
            fn _set_angle(this: &#entity_name, angle: f32);

            #[wasm_bindgen(method, getter, js_name=scaleX)]
            fn _scale_x(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=scaleX)]
            fn _set_scale_x(this: &#entity_name, scale_x: f32);

            #[wasm_bindgen(method, getter, js_name=scaleY)]
            fn _scale_y(this: &#entity_name) -> f32;

            #[wasm_bindgen(method, setter, js_name=scaleY)]
            fn _set_scale_y(this: &#entity_name, scale_y: f32);
        }

        impl crate::object2d::Object2D for #entity_name{
            #[inline(always)]
            fn x(&self) -> f32{
                self._x()
            }

            #[inline(always)]
            fn set_x(&self, x: f32){
                self._set_x(x)
            }

            #[inline(always)]
            fn y(&self) -> f32{
                self._y()
            }

            #[inline(always)]
            fn set_y(&self, y: f32){
                self._set_y(y)
            }

            #[inline(always)]
            fn anchor_x(&self) -> Option<f32> {
                self._anchor_x()
            }

            #[inline(always)]
            fn set_anchor_x(&self, anchor_x: Option<f32>) {
                self._set_anchor_x(anchor_x)
            }

            #[inline(always)]
            fn anchor_y(&self) -> Option<f32>{
                self._anchor_y()
            }

            #[inline(always)]
            fn set_anchor_y(&self, anchor_y: Option<f32>) {
                self._set_anchor_y(anchor_y)
            }

            #[inline(always)]
            fn angle(&self) -> f32{
                self._angle()
            }

            #[inline(always)]
            fn set_angle(&self, angle: f32){
                self._set_angle(angle)
            }

            #[inline(always)]
            fn width(&self) -> f32{
                self._width()
            }

            #[inline(always)]
            fn set_width(&self, w: f32){
                self._set_width(w)
            }

            #[inline(always)]
            fn height(&self) -> f32{
                self._height()
            }

            #[inline(always)]
            fn set_height(&self, h: f32){
                self._set_height(h)
            }

            #[inline(always)]
            fn scale_x(&self) -> f32{
                self._scale_x()
            }

            #[inline(always)]
            fn set_scale_x(&self, scale_x: f32){
                self._set_scale_x(scale_x)
            }

            #[inline(always)]
            fn scale_y(&self) -> f32{
                self._scale_y()
            }

            #[inline(always)]
            fn set_scale_y(&self, scale_y: f32){
                self._set_scale_y(scale_y)
            }

            #[inline(always)]
            fn opacity(&self) -> f32{
                self._opacity()
            }

            #[inline(always)]
            fn set_opacity(&self, opacity: f32){
                self._set_opacity(opacity)
            }

            #[inline(always)]
            fn anchor(&self, x: f32, y: f32){
                self._anchor(x, y)
            }

            #[inline(always)]
            fn move_by(&self, x: f32, y: f32){
               self._move_by(x, y)
            }

            #[inline(always)]
            fn move_to(&self, x: f32, y: f32){
               self._move_to(x, y)
            }

            #[inline(always)]
            fn resize_by(&self, width: f32, height: f32){
                self._resize_by(width, height)
            }

            #[inline(always)]
            fn resize_to(&self, width: f32, height: f32){
                self._resize_to(width, height)
            }

            #[inline(always)]
            fn scale(&self, scale: f32){
                self._scale(scale)
            }
        }
    })
}

