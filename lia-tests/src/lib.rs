#![feature(plugin, const_fn)]
#![plugin(lia_plugin)]

#[macro_use]
extern crate lia;

use lia::runtime::*;

lia! {
    function fib(n) {
        if (n == 0) { return 1; }
        if (n == 1) { return 1; }
        return @fib(n-1) + @fib(n-2);
    }
}

#[test]
fn macro_test() {
    let n = 30;
    let start = Instant::now();
    let result = call!(fib(n));
    cast!(v, result, i32);
    println!("Result: {}", v);
}
