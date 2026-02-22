use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as syn::LitStr);
    let func = parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;

    // skip context arg
    let args: Vec<_> = func.sig.inputs.iter().skip(1).collect();

    let mut arg_names = vec![];

    let arg_parsing = args.iter().map(|arg| {
        let syn::FnArg::Typed(pat_type) = arg else {
            panic!()
        };

        let pat = &pat_type.pat;
        let ty = &pat_type.ty;

        arg_names.push(pat);

        if is_ctx(ty) {
            let inner = ctx_inner_type(ty).unwrap();

            quote! {
              let #pat = {
                  let val = _ctx.data.get::<#inner>()
                      .expect(concat!("missing command data for type: ", stringify!(#inner)))
                      .clone();
                  crate::commands::Ctx(val)
              };
          }
        }
        else {
            let syn::Pat::Ident(ident) = &*pat_type.pat else {
                panic!()
            };

            let name_str = ident.ident.to_string();

            if is_option(ty) {
                quote! {
                  let #pat: #ty = _args.next()
                      .map(|v| <_ as crate::commands::FromArg>::from_arg(#name_str, v))
                      .transpose()?;
              }
            } else {
                quote! {
                  let #pat: #ty = <#ty as crate::commands::FromArg>::from_arg(
                      #name_str,
                      _args.next().ok_or(crate::commands::ArgError::Missing(#name_str))?
                  )?;
              }
            }
        }
    });

    let name_str = name.value();

    quote! {
          #func

          ::inventory::submit! {
              crate::commands::Command {
                  name: #name_str,
                  handler: |_ctx, _raw_args| ::std::boxed::Box::pin(async move {
                      let mut _args = _raw_args.iter().map(String::as_str);
                      #(#arg_parsing)*
                      #func_name(_ctx, #(#arg_names),*).await
                  }),
              }
          }
      }.into()
}

fn is_ctx(ty: &syn::Type) -> bool {
    let syn::Type::Path(p) = ty else {
        return false
    };

    p.path.segments.last().map(|s| s.ident == "Ctx").unwrap_or(false)
}

fn ctx_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(p) = ty else {
        return None
    };

    let seg = p.path.segments.last()?;
    if seg.ident != "Ctx" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(ref args) = seg.arguments else {
        return None
    };

    let syn::GenericArgument::Type(inner) = args.args.first()? else {
        return None
    };

    Some(inner)
}

fn is_option(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false
    };

    type_path.path.segments.last()
        .map(|seg| seg.ident == "Option")
        .unwrap_or(false)
}