fn run_some_closure<F: Fn(i32) -> i32>(f: &F) {
    println!("closure with parameter 6 returned {}", f(6));
}

fn run_mut_closure<F: FnMut(i32)>(f: &mut F) {
    f(12);
}

fn run_once_closure<F: FnOnce() -> String>(f: F) {
    println!("we own string: {}", f());
}

fn main() {
    let plus_one = |x: i32| x + 1;
    run_some_closure(&plus_one);

    let a = 20;
    run_some_closure(&|x| x + a);

    let mut b = 10;

    println!("b before calling closure: {}", b);
    run_mut_closure(&mut |x| {
        b += x;
    });
    println!("b after calling closure: {}", b);

    let s = "hello".to_string();
    let owns_string = move || {
        // s is moved into the closure
        println!("string: {}", s);
        s
    };
    // s is invalid here
    println!("before closure");
    run_once_closure(owns_string);
    println!("after closure");
}
