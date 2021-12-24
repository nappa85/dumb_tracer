
use quote::quote;

#[proc_macro_attribute]
pub fn instrument(
    _args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    //let args = syn::parse_macro_input!(args as InstrumentArgs);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let name = &input.sig.ident;
    let args = input.sig.inputs.iter().map(|arg| match arg {
        syn::FnArg::Receiver(_) => {
            quote!(
                use dumb_tracer::DumbTracerHelper;
                (&mut &self).maybe_debug_print();
            )
        },
        syn::FnArg::Typed(typed) => {
            let name = &typed.pat;
            quote!(
                use dumb_tracer::DumbTracerHelper;
                (&mut &#name).maybe_debug_print();
            )
        },
    }).collect::<proc_macro2::TokenStream>();
    let block = &input.block;
    quote!(
        #(#attrs) *
        #vis #sig {
            eprint!("{}(", stringify!(#name));
            #args
            eprint!(")\n");
            #block
        }
    )
    .into()
}
