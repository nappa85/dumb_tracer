
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
            quote!(write!(handle, ", ").unwrap();)
        }
        else {
            quote!()
        };
        match arg {
            syn::FnArg::Receiver(_) => {
                quote!(
                    write!(handle, "self: ").unwrap();
                    (&mut &self).maybe_debug_print(&mut handle);
                    #comma
                )
            },
            syn::FnArg::Typed(typed) => {
                let name = &typed.pat;
                quote!(
                    write!(handle, "{}: ", stringify!(#name)).unwrap();
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
            let stderr = std::io::stderr();
            let mut handle = stderr.lock();
            write!(handle, "{}(", stringify!(#name)).unwrap();
            #argv
            write!(handle, ")").unwrap();
            let res: #ty = #block;
            write!(handle, " -> ").unwrap();
            (&mut &res).maybe_debug_print(&mut handle);
            writeln!(handle, "").unwrap();
            res
        }
    )
    .into()
}
