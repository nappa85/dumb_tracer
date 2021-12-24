use std::io::Write;

pub use dumb_tracer_macro::instrument;

pub trait DumbTracerHelper {
    fn maybe_debug_print(&self, handle: &mut std::io::StderrLock<'_>);
}

impl<T> DumbTracerHelper for &T {
    fn maybe_debug_print(&self, _: &mut std::io::StderrLock<'_>) {}
}

impl<T: std::fmt::Debug> DumbTracerHelper for &mut &T {
    fn maybe_debug_print(&self, handle: &mut std::io::StderrLock<'_>) {
        write!(handle, "{:?}", self).unwrap();
    }
}

// impl<T: std::fmt::Display> DumbTracerHelper for &mut &mut &T {
//     fn maybe_debug_print(&self) {
//         eprint!("{}", self);    
//     }
// }
