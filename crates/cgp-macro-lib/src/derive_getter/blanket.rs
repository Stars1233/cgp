use alloc::string::ToString;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemImpl, ItemTrait};

use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::{derive_getter_constraint, derive_getter_method, ContextArg};
use crate::symbol::symbol_from_string;

pub fn derive_blanket_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    fields: &[GetterField],
) -> syn::Result<ItemImpl> {
    let consumer_name = &consumer_trait.ident;

    let mut constraints = consumer_trait.supertraits.clone();

    let mut methods: TokenStream = TokenStream::new();

    for field in fields {
        let field_symbol = symbol_from_string(&field.field_name.to_string());

        let method = derive_getter_method(
            &ContextArg::SelfArg,
            field,
            Some(quote! { ::< #field_symbol > }),
            None,
        );

        methods.extend(method);

        let constraint = derive_getter_constraint(field, quote! { #field_symbol })?;

        constraints.push(constraint);
    }

    let (_, type_generics, _) = consumer_trait.generics.split_for_impl();

    let mut item_impl: ItemImpl = parse2(quote! {
        impl< #context_type > #consumer_name #type_generics for #context_type
        where
            #context_type: #constraints
        {
            #methods
        }
    })?;

    item_impl
        .generics
        .params
        .extend(consumer_trait.generics.params.clone());

    if let Some(consumer_where_clause) = &consumer_trait.generics.where_clause {
        item_impl
            .generics
            .make_where_clause()
            .predicates
            .extend(consumer_where_clause.predicates.clone());
    }

    Ok(item_impl)
}
