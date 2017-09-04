#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add_safe(a: u32, b: u32) -> u32 {
    unsafe {
        add(a, b)
    }
}

fn main() {
    println!("{}", add_safe(1, 3));
}
