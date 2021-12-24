
pub use dumb_tracer_macro::instrument;

pub trait DumbTracerHelper {
    fn maybe_debug_print(&self);
}

impl<T> DumbTracerHelper for &T {
    fn maybe_debug_print(&self) {}
}

impl<T: std::fmt::Debug> DumbTracerHelper for &mut &T {
    fn maybe_debug_print(&self) {
        eprint!("{:?}", self);
    }
}

// impl<T: std::fmt::Display> DumbTracerHelper for &mut &mut &T {
//     fn maybe_debug_print(&self) {
//         eprint!("{}", self);    
//     }
// }
