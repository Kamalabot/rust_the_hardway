use proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // parse input to syntax tree 
    let ast = syn::parse(input).unwrap();
    // Generate the implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            // this function is automatically created
            fn hello(&self) {
                println!("Hello, I am a {}", stringify!(#name));
            }
        }
    };

    gen.into()
}



