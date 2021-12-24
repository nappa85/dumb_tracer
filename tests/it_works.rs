
use dumb_tracer::instrument;

#[instrument]
fn foo(bar: &str) {
    println!("{}: it works!", bar);
}

#[test]
fn it_works() {
    foo("bar");
}
