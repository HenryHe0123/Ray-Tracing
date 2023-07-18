extern crate proc_macro;

mod bvh;
mod scene;
mod utility;

use crate::scene::define_static_final_scene;
use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_static_final_scene(_input: TokenStream) -> TokenStream {
    define_static_final_scene()
}
