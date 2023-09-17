use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::object2d::try_expand_object_2d;
use crate::trigger::expand_entity_triggers;

pub mod cacheable;

#[inline]
pub fn expand_entity(input: TokenStream) -> TokenStream {
    try_expand_entity(input)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}


fn try_expand_entity(input: TokenStream) -> syn::Result<TokenStream2> {
    let object_2d = try_expand_object_2d(input.clone())?;
    let entity_name = syn::parse::<ItemStruct>(input)?.ident;
    let entity = expand_impl_entity(&entity_name);
    let triggers = expand_entity_triggers(&entity_name)?;

    Ok(quote! {
        #object_2d
        #entity
        #triggers
    })
}


fn expand_impl_entity(entity_name: &Ident) -> TokenStream2 {
    let impl_into_entity = expand_impl_into_entity(entity_name);

    quote! {
        #impl_into_entity
        
        #[wasm_bindgen(js_namespace=g)]
        extern "C"{
            #[wasm_bindgen(method, getter, js_name=id)]
            fn _id(this: &#entity_name) -> isize;

            #[wasm_bindgen(method, getter, js_name=scene)]
            fn _scene(this: &#entity_name) -> crate::scene::Scene;

            #[wasm_bindgen(method, getter, js_name=game)]
            fn _game(this: &#entity_name) -> crate::game::Game;

            #[wasm_bindgen(method, getter, js_name=children)]
            fn _children(this: &#entity_name) -> Box<[crate::object2d::entity::AkashicEntity]>;

            #[wasm_bindgen(method, getter, js_name=parent)]
            fn _parent(this: &#entity_name) -> wasm_bindgen::JsValue;

            #[wasm_bindgen(method, getter, js_name=touchable)]
            fn _touchable(this: &#entity_name) -> bool;

            #[wasm_bindgen(method, js_name=remove)]
            fn _remove(this: &#entity_name, child_entity: Option<crate::object2d::entity::AkashicEntity>);

            #[wasm_bindgen(method, js_name=append)]
            fn _append(this: &#entity_name, entity: crate::object2d::entity::AkashicEntity);

            #[wasm_bindgen(method, js_name=insertBefore)]
            fn _insert_before(this: &#entity_name, entity: crate::object2d::entity::AkashicEntity, target: Option<crate::object2d::entity::AkashicEntity>);

            #[wasm_bindgen(method, js_name=destroy)]
            fn _destory(this: &#entity_name);

            #[wasm_bindgen(method, getter, js_name=destroyed)]
            fn _destoryed(this: &#entity_name) -> bool;

            #[wasm_bindgen(method, js_name=visible)]
            fn _visible(this: &#entity_name) -> bool;

            #[wasm_bindgen(method, js_name=hide)]
            fn _hide(this: &#entity_name);

            #[wasm_bindgen(method, js_name=modified)]
            fn _modified(this: &#entity_name);
        }

        impl crate::object2d::entity::EntityObject2D for #entity_name{
            #[inline(always)]
            fn id(&self) -> isize{
                self._id()
            }

            #[inline(always)]
            fn scene(&self) -> crate::scene::Scene{
                self._scene()
            }

            #[inline(always)]
            fn game(&self) -> crate::game::Game{
                self._game()
            }

            #[inline(always)]
            fn children(&self) -> Box<[crate::object2d::entity::AkashicEntity]>{
                self._children()
            }

            #[inline(always)]
            fn parent(&self) -> Option<crate::parent::Parent>{
                let parent = self._parent();
                if parent.is_undefined(){
                    return None;
                }

                use wasm_bindgen::prelude::{JsCast, JsValue};
                if let Ok(parent) = parent.clone().dyn_into::<crate::object2d::entity::AkashicEntity>(){
                    return Some(crate::parent::Parent::Entity(parent));
                }

                if let Ok(scene) = parent.dyn_into::<crate::scene::Scene>(){
                    return Some(crate::parent::Parent::Scene(scene));
                }

                panic!("g.Entity.parent type must be Scene or Entity");
            }

            #[inline(always)]
            fn remove_child(&self, target: impl Into<crate::object2d::entity::AkashicEntity>){
                self._remove(Some(target.into()))
            }

            #[inline(always)]
            fn remove_self(&self){
                self._remove(None)
            }

            #[inline(always)]
            fn touchable(&self) -> bool{
                self._touchable()
            }

            #[inline(always)]
            fn append(&self, child: impl Into<crate::object2d::entity::AkashicEntity>){
                self._append(child.into())
            }

            #[inline(always)]
            fn insert_before(&self, child: impl Into<crate::object2d::entity::AkashicEntity>, target: Option<crate::object2d::entity::AkashicEntity>){
                self._insert_before(child.into(), target)
            }

            #[inline(always)]
            fn destroy(&self){
                self._destory()
            }

           #[inline(always)]
            fn destroyed(&self) -> bool{
                self._destoryed()
            }

            #[inline(always)]
            fn visible(&self) -> bool{
                self._visible()
            }

            #[inline(always)]
            fn hide(&self){
                self._hide()
            }

            #[inline(always)]
            fn modified(&self){
                self._modified()
            }

            #[inline(always)]
            fn as_js_value(&self) -> wasm_bindgen::prelude::JsValue{
                self.obj.clone()
            }


            #[inline(always)]
            fn js_value_ref(&self) -> &wasm_bindgen::prelude::JsValue{
                &self.obj
            }
        }
    }
}


fn expand_impl_into_entity(entity_name: &Ident) -> Option<proc_macro2::TokenStream> {
    if *entity_name == "AkashicEntity" {
        None
    } else {
        Some(quote! {
            impl Into<crate::object2d::entity::AkashicEntity> for #entity_name{
                #[inline(always)]
                fn into(self) -> crate::object2d::entity::AkashicEntity{
                    use wasm_bindgen::JsCast;
                    self.unchecked_into()
                }
            }
        })
    }
}
