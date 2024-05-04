use proc_macro::TokenStream;
use quote::quote;
use syn::parse_quote;

#[proc_macro_attribute]
pub fn rune_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // attr: #[rune_impl]
    // item: impl MyStruct { /* ... */ }

    let mut block = dbg!(syn::parse_macro_input!(item as syn::ItemImpl));
    let mut export_list = Vec::new();

    for item in block.items.iter_mut() {
        if let syn::ImplItem::Fn(method) = item {
            let export_attr = parse_quote!(#[export]);
            let exported = method
                .attrs
                .iter()
                .enumerate()
                .find_map(|(index, attr)| (*attr == export_attr).then_some((index, attr)));

            if let Some((index, _)) = exported {
                method.attrs.remove(index);
                export_list.push(method.sig.ident.clone());
            }
        }
    }

    let export_count = export_list.len();
    let exporter = quote! {
        const fn exported_functions() -> [&'static str; #export_count] {
            [#(stringify!(#export_list)),*]
        }
    };

    block.items.push(syn::parse2(exporter).unwrap());

    quote! {
        #block
    }
    .into()
}
