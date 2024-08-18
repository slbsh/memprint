use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MemPrint)]
pub fn memprint_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = if let syn::Data::Struct(ref data) = input.data {
        data.fields.iter().map(|field| &field.ident).collect::<Vec<_>>()
    }
    else {
        panic!("MemPrint can only be derived for structs");
    };

    quote! {
        impl MemPrint for #name {
            fn get_fields(&self) -> Vec<memprint::Field> {
                let mut vec = Vec::new();

                #(
                    let field_offset = (&self.#fields as *const _ as usize) - (self as *const _ as usize);

                    vec.push(memprint::Field::new(
                        stringify!(#fields),
                        (field_offset, field_offset + std::mem::size_of_val(&self.#fields)),
                        &self.#fields
                    ));
                )*

                vec
            }
        }
    }.into()
}

