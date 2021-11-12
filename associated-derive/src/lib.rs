use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Error as ParseError, Parse, ParseStream, Result as ParseResult},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Attribute, Binding, DeriveInput, Expr, Fields, Ident, Type, Variant,
};

struct Args {
    assoc_type: Type,
}

enum AssocKind {
    Constant,
    Static,
}

struct Assoc<'a> {
    kind: AssocKind,
    attr: &'a Attribute,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let b = Binding::parse(input)?;
        if b.ident.to_string() == "Type" {
            return Ok(Args { assoc_type: b.ty });
        }
        Err(ParseError::new(b.ident.span(), "Expected `Type`"))
    }
}

fn generate_match_body(
    enum_ident: &Ident,
    associated_type: &Type,
    associated_variants: &Vec<(&Ident, &Fields, Expr, AssocKind)>,
) -> TokenStream2 {
    let mut match_block = TokenStream2::new();
    match_block.extend(
        associated_variants
            .iter()
            .map(|(variant_ident, fields, expr, kind)| {
                let pattern = match fields {
                    syn::Fields::Named(_) => quote! {{..}},
                    syn::Fields::Unnamed(_) => quote! {(..)},
                    syn::Fields::Unit => quote! {},
                };
                match kind {
                    AssocKind::Constant => {
                        quote! {
                            #enum_ident::#variant_ident #pattern => {
                                const ASSOCIATED: #associated_type = #expr;
                                &ASSOCIATED
                            },
                        }
                    }
                    AssocKind::Static => {
                        quote! {
                            #enum_ident::#variant_ident #pattern => #expr,
                        }
                    }
                }
            }),
    );
    match_block
}

/// Takes in a sequence of enum variants and parses their attributes to return a list of (variant, associated value) groupings.
///
/// Fields are included in the grouping to control which pattern glyph to generate for that variant.
/// AssocKind holds whether the attribute was assoc or assoc_const
fn parse_associated_values<'a>(
    variants: &'a Punctuated<Variant, Comma>,
    enum_ident: &Ident,
) -> Result<Vec<(&'a Ident, &'a Fields, Expr, AssocKind)>, TokenStream> {
    let mut associated_values = Vec::new();
    for v in variants.iter() {
        if let Some(assoc) = v.attrs.iter().find_map(|attr| match attr.path.get_ident() {
            Some(i) => {
                let i = i.to_string();
                if i == "assoc" {
                    Some(Assoc {
                        kind: AssocKind::Static,
                        attr,
                    })
                } else if i == "assoc_const" {
                    Some(Assoc {
                        kind: AssocKind::Constant,
                        attr,
                    })
                } else {
                    None
                }
            }
            None => None,
        }) {
            let expr = match assoc.attr.parse_args::<Expr>() {
                Ok(expr) => expr,
                Err(e) => return Err(e.to_compile_error().into()),
            };

            associated_values.push((&v.ident, &v.fields, expr, assoc.kind));
        } else {
            return Err(ParseError::new(
                v.span(),
                format!(
                    "Cannot derive `Associated` for `{}`: Missing `assoc` or `assoc_const` attribute on variant `{}`",
                    enum_ident.to_string(),
                    v.ident.to_string()
                )
            )
            .to_compile_error()
            .into());
        }
    }
    Ok(associated_values)
}

#[proc_macro_derive(Associated, attributes(associated, assoc, assoc_const))]
pub fn associated_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics: _,
        data,
    } = parse_macro_input!(input);
    let associated = match (&attrs).iter().find(|&attr| match attr.path.get_ident() {
        Some(i) => i.to_string() == "associated",
        None => false,
    }) {
        Some(attr) => attr,
        None => {
            return ParseError::new(ident.span(), "Missing `associated` attribute")
                .to_compile_error()
                .into()
        }
    };
    let args = match associated.parse_args::<Args>() {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    let variants = match data {
        syn::Data::Struct(s) => {
            return ParseError::new(
                s.struct_token.span,
                "Cannot derive `Associated` for structs",
            )
            .to_compile_error()
            .into()
        }
        syn::Data::Union(u) => {
            return ParseError::new(u.union_token.span, "Cannot derive `Associated` for unions")
                .to_compile_error()
                .into()
        }
        syn::Data::Enum(data) => data.variants,
    };
    let associated_variants = match parse_associated_values(&variants, &ident) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let associated_type = args.assoc_type;

    let match_block = generate_match_body(&ident, &associated_type, &associated_variants);

    let impl_block = quote! {
        impl associated::Associated for #ident {
            type AssociatedType = #associated_type;
            fn get_associated(&self) -> &'static Self::AssociatedType {
                match self {
                    #match_block
                }
            }
        }
    };
    impl_block.into()
}
