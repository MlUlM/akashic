use proc_macro::TokenStream;
use convert_case::Case::{Snake, UpperCamel};
use convert_case::Casing;
use proc_macro2::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::ident;

pub fn expand_asset_accessible_traits() -> TokenStream {
    let image = expand_asset_accessible_trait("image", "images");
    let audio = expand_asset_accessible_trait("audio", "audios");
    let text = expand_asset_accessible_trait("text", "texts");
    let output = quote! {
        #image
        #audio
        #text
    };
    output.into()
}

fn expand_asset_accessible_trait(asset_name: &str, plural_asset_name: &str) -> TokenStream2 {
    let upper_name = asset_name.to_case(UpperCamel);
    let single_name = ident(&asset_name.to_case(Snake));
    let plural_name = ident(&plural_asset_name.to_case(Snake));
    let trait_name = ident(&format!("{upper_name}AssetAccessible"));
    let asset_ident = ident(&format!("{upper_name}Asset"));

    quote! {
        paste::paste!{
            pub trait #trait_name{
                fn [<get_ #single_name>](&self, path: impl Into<String>) -> #asset_ident;

                fn [<get_ #single_name _by_id>](&self, asset_id: impl Into<String>) -> #asset_ident;

                fn [<get_all_ #plural_name>](&self) -> Box<[#asset_ident]>;

                fn [<get_all_ #plural_name _with_path_pattern>](&self, path_pattern: String) -> Box<[#asset_ident]>;

                fn [<get_all_ #plural_name _with_filter>](&self, filter: impl 'static + FnMut(String) -> bool) -> Box<[#asset_ident]>;
            }
        }
    }
}


pub fn expand_asset_accessor_derive(input: TokenStream) -> TokenStream {
    syn::parse::<ItemStruct>(input)
        .map(|item| {
            let entity_name = &item.ident;
            let image = expand_asset_accessor_derive_from_name(entity_name, "image", "Images");
            let audio = expand_asset_accessor_derive_from_name(entity_name, "audio", "Audios");
            let text = expand_asset_accessor_derive_from_name(entity_name, "text", "Texts");

            quote! {
                #image
                #audio
                #text
            }
        })
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_asset_accessor_derive_from_name(
    entity_name: &Ident,
    asset_name: &str,
    plural_asset_name: &str,
) -> TokenStream2 {
    let upper_name = asset_name.to_case(UpperCamel);
    let single_snake_case = ident(&asset_name.to_case(Snake));
    let plural_snake_case = ident(&plural_asset_name.to_case(Snake));

    let trait_name = ident(&format!("{upper_name}AssetAccessible"));
    let asset_ident = ident(&format!("{upper_name}Asset"));
    let single = ident(&upper_name);
    let plural = ident(plural_asset_name);

    quote! {
        paste::paste!{
            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = g)]
            extern {
                #[wasm_bindgen::prelude::wasm_bindgen(method, js_name = [<get #single>])]
                fn [<_get_ #single_snake_case>](this: &#entity_name, path: String) -> #asset_ident;

                #[wasm_bindgen::prelude::wasm_bindgen(method, js_name = [<get #single ById>])]
                fn [<_get_ #single_snake_case _by_id>](this: &#entity_name, asset_id: String) -> #asset_ident;

                #[wasm_bindgen::prelude::wasm_bindgen(method, js_name = [<getAll #plural>])]
                fn [<_get_all_ #plural_snake_case>](this: &#entity_name) -> Box<[#asset_ident]>;

                #[wasm_bindgen::prelude::wasm_bindgen(method, js_name = [<getAll #plural>])]
                fn [<_get_all_ #plural_snake_case _with_path_pattern>](this: &#entity_name, path_pattern: String) -> Box<[#asset_ident]>;

                #[wasm_bindgen::prelude::wasm_bindgen(method, js_name = [<getAll #plural>])]
                fn [<_get_all_ #plural_snake_case _with_filter>](this: &#entity_name, filter: wasm_bindgen::JsValue) -> Box<[#asset_ident]>;
            }


            impl #trait_name for #entity_name{
                fn [<get_ #single_snake_case>](&self, path: impl Into<String>) -> #asset_ident{
                    self.[<_get_ #single_snake_case>](path.into())
                }

                fn [<get_ #single_snake_case _by_id>](&self, asset_id: impl Into<String>) -> #asset_ident{
                    self.[<_get_ #single_snake_case _by_id>](asset_id.into())
                }

                fn [<get_all_ #plural_snake_case>](&self) -> Box<[#asset_ident]>{
                    self.[<_get_all_ #plural_snake_case>]()
                }

                fn [<get_all_ #plural_snake_case _with_path_pattern>](&self, path_pattern: String) -> Box<[#asset_ident]>{
                    self.[<_get_all_ #plural_snake_case _with_path_pattern>](path_pattern)
                }

                fn [<get_all_ #plural_snake_case _with_filter>](&self, filter: impl 'static + FnMut(String) -> bool) -> Box<[#asset_ident]>{
                    use crate::util::FunctionIntoJsValue;
                    let input = (Box::new(filter) as Box<dyn FnMut(String) -> bool>).into_js_value();
                    self.[<_get_all_ #plural_snake_case _with_filter>](input)
                }
            }
        }
    }
}