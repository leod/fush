use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, FnArg, ItemFn, Pat, Result};

pub fn transform(mut item: ItemFn) -> Result<TokenStream2> {
    let mut input_idents = Vec::new();
    let mut input_tys = Vec::new();

    for input in item.sig.inputs.iter_mut() {
        if let FnArg::Typed(input) = input {
            match &*input.pat {
                Pat::Ident(ident) => {
                    input_idents.push(ident.ident.clone());
                    input_tys.push(input.ty.clone());

                    let input_ty = &input.ty;
                    input.ty = parse_quote! { impl ::posh::value::IntoValue<Value = #input_ty> };
                }
                _ => {
                    return Err(Error::new_spanned(
                        &input.pat,
                        "posh: Only identifiers are allowed as function argument patterns",
                    ));
                }
            }
        }
    }

    let args_ident = quote! { __posh_func_args };

    let func_ident = item.sig.ident.clone();
    let func_body = item.block.clone();

    item.block = parse_quote! {
        {
            use ::posh::Value as _;

            const _: fn() = || {
                use ::posh::static_assertions as sa;

                #(
                    sa::assert_impl_all!(#input_tys: ::posh::Value);
                    sa::assert_impl_all!(
                        <#input_tys as ::posh::Value>::Type:
                        ::posh::value::FuncArg,
                    );
                )*
            };

            #(
                let #input_idents = ::posh::IntoValue::into_value(#input_idents);
            )*

            let #args_ident = vec![
                #(
                    ::posh::Value::expr(&#input_idents).clone()
                ),*
            ];

            #(
                let #input_idents =
                    ::posh::Value::with_expr(
                        &#input_idents,
                        ::posh::lang::Expr::Var(::posh::lang::VarExpr {
                            ident: ::posh::lang::Ident::new(stringify!(#input_idents)),
                            ty: ::posh::value::Value::ty(&#input_idents),
                            init: None,
                        }),
                    );
            )*

            ::posh::value::func_call(
                stringify!(#func_ident),
                vec![
                    #(
                        ::posh::lang::VarExpr {
                            ident: ::posh::lang::Ident::new(stringify!(#input_idents)),
                            ty: ::posh::value::Value::ty(&#input_idents),
                            init: None,
                        }
                    ),*
                ],
                {
                    use ::posh::prelude::*;
                    ::posh::IntoValue::into_value(#func_body)
                },
                #args_ident,
            )
        }
    };

    Ok(item.into_token_stream())
}
