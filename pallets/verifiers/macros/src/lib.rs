// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate expose some procedural macro utils for implementing
//! a new verifier pallet based on `pallet-verifiers` abstraction.
//!

use proc_macro_crate::FoundCrate;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Attribute, Ident, Token, Visibility};

struct Item {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub semi_token: Token![;],
}

impl syn::parse::Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let struct_token = input.parse()?;
        let ident = input.parse()?;
        let semi_token = input.parse()?;
        Ok(Item {
            attrs,
            vis,
            struct_token,
            ident,
            semi_token,
        })
    }
}

/// The attribute `#[verifier]` can be used on a new struct that should implement
/// `pallet-verifier::Verifier` trait: will generate the need type aliases and
/// reexport the `pallet-verifiers` substrate generated stuff needed to
/// use this crate or module as the home of the new pallet.
///
/// It accept only structs without fields and generics.
///
#[proc_macro_attribute]
pub fn verifier(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    verifier_render(parse_macro_input!(item as Item))
}

fn verifier_render(item: Item) -> proc_macro::TokenStream {
    let Item {
        attrs,
        vis,
        struct_token,
        ident,
        semi_token,
    } = item;
    let crate_name = crate_name();
    quote! {
        #(#attrs)*
        #vis #struct_token #ident #semi_token
        pub type Pallet<T> = #crate_name::Pallet<T, #ident>;
        pub type Event<T> = #crate_name::Event<T, #ident>;
        pub type Error<T> = #crate_name::Error<T, #ident>;
        pub use #crate_name::{
            __substrate_call_check, __substrate_event_check, tt_default_parts, tt_error_token,
        };
    }
    .into()
}

fn crate_name() -> syn::Path {
    match proc_macro_crate::crate_name("pallet-verifiers")
        .expect("pallet-verifiers is present in `Cargo.toml` qed")
    {
        FoundCrate::Itself => parse_quote! { crate },
        FoundCrate::Name(name) => {
            let myself = format_ident!("{name}");
            parse_quote! { #myself }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note: here we test just the parsing stuff. Logic and functionalities are tested
    // in the `pallet-verifiers` crate (the `FakeVerifier` in mock module use this macro)
    // .

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("pub struct Verifier;")]
    #[case("pub struct Other;")]
    #[case::no_pub("struct Verifier;")]
    #[case::comments(
        r#"
    /// comm
    /// ents
    pub struct Verifier;"#
    )]
    fn should_parse_valid_item(#[case] input: &str) {
        assert!(syn::parse_str::<Item>(input).is_ok())
    }

    #[rstest]
    #[case::named_tuple("struct Verifier(Other);")]
    #[case::field("struct Other{a: u32}")]
    #[case::generics("struct Verifier<A>;")]
    #[case::enum_type("enum Verifier;")]
    fn should_reject_invalid_item(#[case] input: &str) {
        assert!(syn::parse_str::<Item>(input).is_err())
    }
}
