use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemConst, parse_macro_input};

#[proc_macro_attribute]
pub fn const_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the const item
    let const_item = parse_macro_input!(item as ItemConst);
    let ident = &const_item.ident;
    let expr = &const_item.expr;
    let const_tokens = const_item.to_token_stream();

    // Build a test function name (export_{IDENT})
    let test_fn_name = syn::Ident::new(&format!("export_bindings_{}", ident), ident.span());

    let out_path = quote! { std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("constants.ts") };

    // Generate the test function that appends one line like:
    // export const CHANNEL = "app://activity";
    // We'll evaluate the const expression at runtime in the test and write it with Debug ({:?}) which
    // produces a quoted string for &str, unquoted numbers/bools.
    let expanded = quote! {
        #const_tokens

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
            let line = format!("export const {} = {:?};\n", stringify!(#ident), value);

            file.write_all(line.as_bytes()).expect("failed to write constant export");
        }
    };

    TokenStream::from(expanded)
}
