use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::path::PathBuf;
use syn::{
    Attribute, ItemConst, LitStr, meta::ParseNestedMeta, parse_macro_input, spanned::Spanned,
};

/// Extract doc comments from attributes and return a formatted JSDoc block as LitStr.
///
/// If there are no doc attributes, returns None.
fn extract_doc_as_jsdoc(attrs: &[Attribute]) -> Option<String> {
    // Collect all `#[doc = "..."]` values in order
    let mut lines: Vec<String> = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("doc")) {
        if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
            if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                    lines.push(lit_str.value());
                }
            }
        }
    }

    if lines.is_empty() {
        return None;
    }

    // Sanitize lines (trim trailing whitespace but keep leading for indentation if needed)
    // Escape any closing-comment sequences to avoid prematurely ending the JSDoc.
    let sanitized_lines: Vec<String> = lines
        .into_iter()
        .map(|l| l.replace("*/", "*\\/")) // escape `*/` inside doc text
        .collect();

    // Prefix each line with " * " for JSDoc style
    let body = sanitized_lines
        .into_iter()
        .map(|l| format!(" * {}", l))
        .collect::<Vec<_>>()
        .join("\n");

    let jsdoc = format!("/**\n{}\n */\n", body);

    Some(jsdoc)
}

#[derive(Default)]
struct ConstExportAttributes {
    export_to: Option<PathBuf>,
}

impl ConstExportAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("export_to") {
            let path_lit: LitStr = meta.value()?.parse()?;
            self.export_to = Some(path_lit.value().into());
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

    if attrs.export_to.is_none() {
        return quote! { compile_error!("expected path attribute"); }.into();
    }

    // Parse the const item
    let const_item = parse_macro_input!(item as ItemConst);

    let doc_comments = extract_doc_as_jsdoc(const_item.attrs.as_slice()).unwrap_or_default();

    let ident = &const_item.ident;
    let expr = &const_item.expr;
    let const_tokens = const_item.to_token_stream();

    // Build a test function name (export_{IDENT})
    let test_fn_name = syn::Ident::new(&format!("export_bindings_{}", ident), ident.span());

    let path = attrs.export_to.unwrap().to_string_lossy().to_string();
    let out_path =
        quote! { std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("bindings").join(#path) };

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
            let doc = #doc_comments;

            std::fs::create_dir_all(out.parent().unwrap()).unwrap();

            // open in append mode
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&out)
                .expect("failed to open constants output file");

            let value = #expr;
            // write using debug formatter so strings get quoted
            let line = format!("{}export const {} = {:?};\n", doc, stringify!(#ident), value);

            file.write_all(line.as_bytes()).expect("failed to write constant export");
        }
    };

    TokenStream::from(expanded)
}
