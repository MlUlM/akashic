pub mod cache;

use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::children::expand_children;
use crate::modified::expand_modify;
use crate::trigger::expand_entity_triggers;

#[inline]
pub fn expand_entity(input: TokenStream) -> TokenStream {
    try_expand_entity(input).unwrap_or_else(|e| e.into_compile_error().into())
}


fn try_expand_entity(input: TokenStream) -> syn::Result<TokenStream> {
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let entity = expand_impl_entity(&entity_name);
    let triggers = expand_entity_triggers(&entity_name)?;

    Ok(quote! {
        #entity
        #triggers
    }.into())
}


fn expand_impl_entity(entity_name: &Ident) -> TokenStream2 {
    let children = expand_children(entity_name);
    let modify = expand_modify(entity_name);
    let size = expand_entity_size(entity_name);
    let destroy = expand_entity_destroy(entity_name);
    let angle = expand_entity_angle(entity_name);
    let impl_into_entity = expand_impl_into_entity(entity_name);
    let append = expand_append(entity_name);

    quote! {
        #modify
        #children
        #size
        #destroy
        #angle
        #append
        #impl_into_entity
        
        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method, getter, js_name=id)]
            fn _id(this: &#entity_name) -> usize;

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn x(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, setter = x)]
            pub fn set_x(this: &#entity_name, x: f32);

            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn y(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, setter = y)]
            pub fn set_y(this: &#entity_name, y: f32);
        }

        impl crate::entity::E for #entity_name{
            #[inline(always)]
            fn id(&self) -> usize{
                self._id()
            }

            #[inline(always)]
            fn as_js_value(&self) -> wasm_bindgen::JsValue{
                self.obj.clone()
            }


        }



    }
}





pub fn expand_entity_size(
    entity_name: &Ident
) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[doc(hidden)]
            #[wasm_bindgen(js_namespace = g, method, getter, js_name=width)]
            fn _width(this: &#entity_name) -> f32;

            #[doc(hidden)]
            #[wasm_bindgen(js_namespace = g, method, setter, js_name=width)]
            fn _set_width(this: &#entity_name, width: f32);

            #[doc(hidden)]
            #[wasm_bindgen(js_namespace = g, method, getter, js_name=height)]
            fn _height(this: &#entity_name) -> f32;

            #[doc(hidden)]
            #[wasm_bindgen(js_namespace = g, method, setter, js_name=height)]
            fn _set_height(this: &#entity_name, height: f32);
        }


        impl crate::entity::EntitySize for #entity_name{
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

        }
    }
}


pub fn expand_entity_destroy(
    entity_name: &Ident
) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[doc(hidden)]
            #[wasm_bindgen(js_namespace = g, method,  js_name=destroy)]
            fn _destory(this: &#entity_name, destroySurface: bool);
        }


        impl crate::entity::EntityDestroy for #entity_name{
            #[inline(always)]
            fn destroy(&self){
                self._destory(false)
            }

            #[inline(always)]
            fn destroy_with_surface(&self){
                self._destory(true)
            }
        }
    }
}


fn expand_entity_angle(
    entity_name: &Ident
) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method, getter)]
            pub fn angle(this: &#entity_name) -> f32;

            #[wasm_bindgen(js_namespace = g, method, setter, js_name=angle)]
            pub fn set_angle(this: &#entity_name, angle: f32);
        }
    }
}


fn expand_append(
    entity_name: &Ident
) -> TokenStream2 {
    quote! {
        #[wasm_bindgen]
        extern "C"{
            #[wasm_bindgen(js_namespace = g, method)]
            fn _append(this: &#entity_name, entity: crate::entity::Entity);
        }


        impl crate::entity::AppendEntity for #entity_name{
            #[inline(always)]
            fn append(&self, child: impl Into<crate::entity::Entity>){
                self._append(child.into());
            }
        }
    }
}


fn expand_impl_into_entity(entity_name: &Ident) -> Option<proc_macro2::TokenStream> {
    if *entity_name == "Entity"{
        None
    }else{
        Some(quote!{
            impl Into<crate::entity::Entity> for #entity_name{
                #[inline(always)]
                fn into(self) -> crate::entity::Entity{
                    use wasm_bindgen::JsCast;
                    self.unchecked_into()
                }
            }
        })
    }
}