// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    // let x : i8 = 5;
    // call_me(x.into());
    call_me(5)
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
