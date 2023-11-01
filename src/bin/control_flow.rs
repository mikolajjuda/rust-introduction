fn main() {
    let x = -2;
    {
        // blocks sequentially execute statements
        let x = 10; // item declarations in blocks are scoped to the block
        println!("x is {}", x);
        // default return value of a block is ()
    }
    println!("x is {}", x);
    let y = {
        let y = 2;
        y * y - x
        /* if the final expression is not followed by a semicolon
        it becomes a return value of a block
        */
    };
    println!("y is {}", y);

    println!();

    if x < 0 {
        println!("x is negative");
    } else if x > 0 {
        println!("x is positive");
    } else {
        println!("x is zero");
    }
    let y = if x < 0 { -x } else { x };
    println!("y is {}", y);

    let mut a = 1;
    let b = loop {
        a *= 2;
        if a >= 10 {
            break a;
        }
    };
    println!("first power of 2 greater than 10 is {}", b);

    println!();

    let mut a = 1;
    while a <= 3 {
        println!("a is {}", a);
        a += 1;
    }

    println!();

    for i in 0..5 {
        println!("{}", i);
    }
    let arr = ["dog", "cat", "horse"];
    for animal in arr {
        println!("{}", animal);
    }

    println!();

    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            break 'outer;
        }
        // println!("never printed");
    }

    let a = 'some_block_label: {
        let a = true;
        if a {
            break 'some_block_label false;
        } else {
            break 'some_block_label true;
        }
    };
    println!("a is {}", a);

    println!();

    for i in 0..=5 {
        if i < 5 {
            continue;
        }
        println!("{}", i);
    }
}
