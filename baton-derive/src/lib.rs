//! Derive a struct, creatig a `XxxSend` and `XxxRecv` struct.
//! Each field has its own channel, allowing you to send and receive updates independently.
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/*
// example:
#[derive(Baton)]
struct MyStruct {
    pub field1: i32,
    pub field2: String,
}

// expands to:
struct MyStructSend {
    pub field1: baton::Send<i32>,
    pub field2: baton::Send<String>,
}

struct MyStructRecv {
    pub field1: baton::Recv<i32>,
    pub field2: baton::Recv<String>,
}

impl MyStruct {
    fn baton(self) -> (MyStructSend, MyStructRecv) {
        // ...
    }
}
 */

#[proc_macro_derive(Baton)]
pub fn derive_baton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let base_name = input.ident;
    let vis = input.vis;

    // Define names for generated send/recv structs
    let send_name = syn::Ident::new(&format!("{}Send", base_name), base_name.span());
    let recv_name = syn::Ident::new(&format!("{}Recv", base_name), base_name.span());

    // Extract fields from the struct
    let fields = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Baton can only be derived for structs with named fields"),
        }
    } else {
        panic!("Baton can only be derived for structs");
    };

    // Generate fields for send/recv structs
    let send_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis #name: ::baton::Send<#ty>,
        }
    });

    let recv_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis #name: ::baton::Recv<#ty>,
        }
    });

    // Generate the baton function
    let baton_fn_fields = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            let #name = ::baton::channel(self.#name);
        }
    });

    let baton_fn_return_send = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: #name.0,
        }
    });

    let baton_fn_return_recv = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: #name.1,
        }
    });

    // Generate the output tokens
    let expanded = quote! {
        #vis struct #send_name {
            #(#send_fields)*
        }

        #vis struct #recv_name {
            #(#recv_fields)*
        }

        impl #base_name {
            pub fn baton(self) -> (#send_name, #recv_name) {
                #(#baton_fn_fields)*

                (
                    #send_name {
                        #(#baton_fn_return_send)*
                    },
                    #recv_name {
                        #(#baton_fn_return_recv)*
                    }
                )
            }
        }
    };

    TokenStream::from(expanded)
}
