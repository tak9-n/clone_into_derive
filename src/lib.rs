extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::Ident;
use proc_macro2::Span;

#[proc_macro_derive(CloneInto)]
pub fn clone_into_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_define_clone_into_macro(&ast)
}

fn make_macro_name(camel: &Ident) -> Ident {
    let word_re = Regex::new(r"[A-Z][a-z]+").unwrap();
    let snake = word_re.captures_iter(&camel.to_string()).fold("".to_string(), |acc, w| {
        let i = &w[0].to_lowercase();
        if acc.len() > 0 {
            format!("{}_{}",acc, i)
        } else {
            i.to_string()
        }
    });
    Ident::new(&format!("{}_clone_into",snake), Span::call_site())
}

fn impl_define_clone_into_macro(ast: &syn::DeriveInput) -> TokenStream {
    let macro_name = make_macro_name(&ast.ident);
    match &ast.data {
        syn::Data::Struct(v) => {
            let field_names = v.clone().fields.into_iter().filter_map(
                |field|
                match field.vis {
                    syn::Visibility::Public(_) => Some(field.ident),
                    _ => None,
                });
            let field_names2 = field_names.clone();
            let out = quote! {
                #[macro_export]
                macro_rules! #macro_name {
                    ($from: ident, $to: ident { $($id: ident : $val: expr)* }) => {
                        $to {
                            #( #field_names2 : $from.#field_names2.clone() , )*
                            $($id : $val, )*
                        }
                    };
                    ($from: ident, $to: ident) => {
                        #( $to.#field_names = $from.#field_names.clone() );*
                    };
                }
            };
            out.into()
        },
        _ => {
            syn::Error::new_spanned(
                &ast.ident,
                "Must be struct type",
            ).to_compile_error().into()
        }
    }
}

