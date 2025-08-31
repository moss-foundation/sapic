use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::path::PathBuf;
use syn::{ItemConst, LitStr, meta::ParseNestedMeta, parse_macro_input, spanned::Spanned};

#[derive(Default)]
struct ConstExportAttributes {
    path: Option<PathBuf>,
}

impl ConstExportAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("path") {
            let path_lit: LitStr = meta.value()?.parse()?;
            self.path = Some(path_lit.value().into());
            Ok(())
        } else {
            Err(syn::Error::new(meta.path.span(), "unsupported attribute"))
        }
    }
}

/// #[const_export(path="constants.ts")]
#[proc_macro_attribute]
pub fn const_export(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut attrs = ConstExportAttributes::default();
    let attr_parser = syn::meta::parser(|meta| attrs.parse(meta));
    parse_macro_input!(args with attr_parser);

    if attrs.path.is_none() {
        return quote! { compile_error!("expected path attribute"); }.into();
    }

    // Parse the const item
    let const_item = parse_macro_input!(item as ItemConst);
    let ident = &const_item.ident;
    let expr = &const_item.expr;
    let const_tokens = const_item.to_token_stream();

    // Build a test function name (export_{IDENT})
    let test_fn_name = syn::Ident::new(&format!("export_bindings_{}", ident), ident.span());

    let path = attrs.path.unwrap().to_string_lossy().to_string();
    let out_path = quote! { std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(#path) };

    // Generate the test function that appends one line like:
    // export const CHANNEL = "app://activity";
    // We'll evaluate the const expression at runtime in the test and write it with Debug ({:?}) which
    // produces a quoted string for &str, unquoted numbers/bools.
    let expanded = quote! {
        #const_tokens

        #[allow(non_snake_case)]
        #[cfg(test)]
        #[test]
        fn #test_fn_name() {
            use std::fs::OpenOptions;
            use std::io::Write;

            let out = #out_path;

            // open in append mode
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&out)
                .expect("failed to open constants output file");

            let value = #expr;
            // write using debug formatter so strings get quoted


            let line = format!("/**\n * @category Constant\n */\nexport const {} = {:?};\n", stringify!(#ident), value);

            file.write_all(line.as_bytes()).expect("failed to write constant export");
        }
    };

    TokenStream::from(expanded)
}
