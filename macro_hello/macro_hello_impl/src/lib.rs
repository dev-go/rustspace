extern crate proc_macro;

#[proc_macro_derive(MacroHello)]
pub fn macro_hello_impl(input: crate::proc_macro::TokenStream) -> crate::proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote::quote! {
        impl MacroHello for #name {
            fn macro_hello() {
                println!("Hello, Macro! name = {:?}", stringify!(#name));
            }
        }
    };
    gen.into()
}
