// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

fn fn_call_me_2(f: &Fn(i32), arg: i32) {
    f(arg);
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn function2(i: i32) {
    println!("I'm a function ({})", i);
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    let closure2 = |i:i32| println!("I'm a closure ({})", i);

    fn_call_me_2(&function2, 20);
    fn_call_me_2(&closure2, 30);
}
