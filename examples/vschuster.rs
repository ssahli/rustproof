#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let x = add_five(1, true);
}

// WP=      x+5 = x+5 & x < MAX-5
// #[condition(pre="x < MAX-5", post="return = x+5")]

#[condition(pre="x: int < 2147483642", post="return: int == x: int +5")]
fn add_five(x: i32, y: bool) -> i32 {
    if y
        {x + 5}
    else
        {x}
}
