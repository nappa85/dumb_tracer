# dumb_tracer
Worst tracer in town

## Warning
Really, this is the dumbest tracer ever seen. Don't use it

## What is does
It simply prints to stderr every call to a decorated method, with parameters, if they suppot Debug printing

## Example
```rust
use dumb_tracer::instrument;

#[instrument]
fn foo(bar: &str) {
    println!("{}: it works!", bar);
}

fn main() {
    foo("bar");
}
```

This prints to stderr something like
```
foo(bar: "bar") -> () 
```

## Credits
Thanks to @Rimpampa and @dodomorandi for the negative trait solution.
