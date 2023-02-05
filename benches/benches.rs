#![feature(test)]

extern crate test;
extern crate wasm_rust;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_rust::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}