use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, token::{Brace, Token}, Field, Member};

#[proc_macro_derive(MemPrint)]
pub fn memprint_derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as syn::ItemStruct);

	let name = &input.ident;

	let fields = input.fields.iter()
		.enumerate()
		.map(|(i, f)| f.ident.as_ref().map_or_else(
			||  Member::Unnamed(syn::Index { index: i as u32, span: Span::call_site() }),
			|f| Member::Named(f.clone())))
		.collect::<Vec<Member>>();

	let dtype_ident = syn::Ident::new(match () {
		_ if fields.is_empty()          => "UnitStruct",
		_ if input.semi_token.is_some() => "TupleStruct",
		_ => "Struct", // TODO: make the work for enums & onions
	}, Span::call_site());

	let mut body = quote! {
		fn data_type(&self) -> memprint::DataType {
			memprint::DataType::#dtype_ident
		}
	};
	
	if matches!(dtype_ident.to_string().as_str(), "Struct" | "TupleStruct") {
		body.extend(quote! {
			fn fields(&self) -> Option<Vec<memprint::Field>> {
				let mut vec = Vec::new();

				#(
					let field_offset = (&self.#fields as *const _ as usize) - (self as *const _ as usize);

					vec.push(memprint::Field::new(
						stringify!(#fields),
						(field_offset, field_offset + std::mem::size_of_val(&self.#fields)),
						&self.#fields
					));
				)*

				Some(vec)
			}
		});
	}

	let out = quote! {
		impl MemPrint for #name {
			#body
		}
	};

	out.into()
}

