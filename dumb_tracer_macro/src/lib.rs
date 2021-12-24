
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
    let argc = input.sig.inputs.len();
    let argv = input.sig.inputs.iter().enumerate().map(|(index, arg)| {
        let comma = if index + 1 < argc {
            quote!(write!(&mut handle, ", ").unwrap();)
        }
        else {
            quote!()
        };
        match arg {
            syn::FnArg::Receiver(_) => {
                quote!(
                    write!(&mut handle, "self: ").unwrap();
                    (&mut &self).maybe_debug_print(&mut handle);
                    #comma
                )
            },
            syn::FnArg::Typed(typed) => {
                let name = &typed.pat;
                quote!(
                    write!(&mut handle, "{}: ", stringify!(#name)).unwrap();
                    (&mut &#name).maybe_debug_print(&mut handle);
                    #comma
                )
            },
        }
    }).collect::<proc_macro2::TokenStream>();
    let ty = match input.sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ref t) => quote!(#t),
    };
    let block = &input.block;
    quote!(
        #(#attrs) *
        #vis #sig {
            use dumb_tracer::DumbTracerHelper as _;
            use std::io::Write as _;
            let mut buffer: Vec<u8> = Vec::with_capacity(1999999);
            let mut handle = std::io::Cursor::new(&mut buffer);
            write!(&mut handle, "{}(", stringify!(#name)).unwrap();
            #argv
            write!(&mut handle, ")").unwrap();
            let res: #ty = #block;
            write!(&mut handle, " -> ").unwrap();
            (&mut &res).maybe_debug_print(&mut handle);
            writeln!(&mut handle, "").unwrap();
            std::io::stderr().write_all(handle.into_inner()).unwrap();
            res
        }
    )
    .into()
}
