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
    field1: tokio::watch::Sender<i32>,
    field2: tokio::watch::Sender<String>,
}

struct MyStructRecv {
    field1: tokio::watch::Receiver<i32>,
    field2: tokio::watch::Receiver<String>,
}

impl MyStruct {
    fn baton(self) -> (MyStructSend, MyStructRecv) {
        // ...
    }
}

impl MyStructSend {
    pub fn field1(value: i32) -> Result<(), baton::Closed> {
        self.field1.send(value).is_ok()
    }

    pub fn field2(value: String) -> Result<(), baton::Closed> {
        self.field2.send(value).is_ok()
    }
}

impl MyStructRecv {
    pub async fn field1(&self) -> Result<i32, baton::Closed> {
        self.field1.changed().await.map_err(|_| baton::Closed)
        self.field1.borrow_and_update().clone()
    }

    pub async fn field2(&self) -> Result<String, baton::Closed> {
        self.field2.changed().await.map_err(|_| baton::Closed)
        self.field2.borrow_and_update().clone()
    }
}

 */

#[proc_macro_derive(Baton)]
pub fn derive_baton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let base_name = input.ident;

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
            #vis #name: ::tokio::sync::watch::Sender<#ty>,
        }
    });

    let recv_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis #name: ::std::sync::Arc<::tokio::sync::Mutex<::tokio::sync::watch::Receiver<#ty>>>,
        }
    });

    // Generate the baton function
    let baton_fn_fields = fields.iter().map(|f| {
        let name = &f.ident;

        quote! {
            let mut #name = ::tokio::sync::watch::channel(self.#name);
            #name.1.mark_changed();
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
            #name: ::std::sync::Arc::new(::tokio::sync::Mutex::new(#name.1.into())),
        }
    });

    // Generate send methods
    let send_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis fn #name(&self, value: #ty) -> Option<()> {
                self.#name.send(value).ok()
            }
        }
    });

    // Generate recv methods
    let recv_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;
        quote! {
            #vis async fn #name(&self) -> Option<#ty> {
                let mut lock = self.#name.lock().await;
                lock.changed().await.ok()?;
                let value = lock.borrow_and_update().clone();
                Some(value)
            }
        }
    });

    // Generate the output tokens
    let expanded = quote! {
        #[derive(Clone)]
        pub struct #send_name {
            #(#send_fields)*
        }

        #[derive(Clone)]
        pub struct #recv_name {
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

        impl #send_name {
            #(#send_methods)*
        }

        impl #recv_name {
            #(#recv_methods)*
        }
    };

    TokenStream::from(expanded)
}
