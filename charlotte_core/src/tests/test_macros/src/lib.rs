extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::ItemFn;

fn impl_test_macro(f: ItemFn) -> TokenStream {
    let name = &f.sig.ident;
    let name_str = name.to_string();
    let obj_name = format_ident!("__TEST_DESCR_{}", name);

    let block = f.block.clone();

    let gen = quote! {
        #[used]
        #[link_section = ".kernel_tests"]
        static #obj_name: crate::tests::TestDescription = crate::tests::TestDescription {
            name: #name_str,
            func: #name,
            module: module_path!()
        };

        fn #name() -> bool {
            #block
            true
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn kernel_test(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let expanded = impl_test_macro(syn::parse_macro_input!(item));

    TokenStream::from(expanded)
}
