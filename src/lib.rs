use std::io::Write;

pub use dumb_tracer_macro::instrument;

pub trait DumbTracerHelper {
    fn maybe_debug_print(&self, handle: &mut std::io::Cursor<&mut Vec<u8>>);
}

impl<T> DumbTracerHelper for &T {
    fn maybe_debug_print(&self, _: &mut std::io::Cursor<&mut Vec<u8>>) {}
}

impl<T: std::fmt::Debug> DumbTracerHelper for &mut &T {
    fn maybe_debug_print(&self, handle: &mut std::io::Cursor<&mut Vec<u8>>) {
        write!(handle, "{:?}", self).unwrap();
    }
}

// impl<T: std::fmt::Display> DumbTracerHelper for &mut &mut &T {
//     fn maybe_debug_print(&self) {
//         eprint!("{}", self);    
//     }
// }
