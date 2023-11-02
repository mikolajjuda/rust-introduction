struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(RGB),
    HSV{h: u8, s: u8, v: u8},
    CMYK(u8, u8, u8, u8),
}

fn main() {
    let number = 7;
    match number {
        1 => println!("lonely"),
        2 | 3 | 5 | 7 | 11 => println!("small prime"),
        13..=19 => println!("teen"),
        x if x < 0 => println!("negative"),
        _ => println!("not special"),
    }
    let a = true;
    let b = match a {
        true => 1,
        false => 0,
    };
    println!("b is {}", b);

    println!();

    let c = RGB { r: 64, g: 0, b: 0 };
    match c {
        RGB { r: 0, g: 0, b: 0 } => println!("black"),
        RGB { r: r @ 0..=80, g: 0, b: 0, } => println!("some sort of dark red {r}"),
        RGB { b: 255, .. } => println!("something with maximum blue"),
        RGB { r, g, b } => println!("r: {}, g: {}, b: {}", r, g, b),
    }
    let RGB { r: red, g: green, b: blue, } = c;
    println!("red: {}, green: {}, blue: {}", red, green, blue);

    println!();

    let some_tuple: (u8, bool, f32, char) = (10, false, 0.1, 'Σ');
    match some_tuple {
        (0..=9, .., 'α'..='ω' | 'Α'..='Ω') => println!("digit and a greek letter"),
        (.., 'Σ') => println!("ends with a letter sigma"),
        (0, ..) => println!("starts with zero"),
        (_, true, _, c) => println!("second value is true and fourth is {}", c),
        entire_tuple => println!("{:?}", entire_tuple),
    }
    let (a, c, d, e) = some_tuple;
    println!("a: {}, c: {}, d: {}, e: {}", a, c, d, e);

    println!();

    let arr = [1, 2, 3, 4, 5];
    match arr {
        [0, .., last] => println!("arr[0]: 0, last: {}", last),
        [first, middle @ .., 5] => println!("arr[0]: {}, middle: {:?}", first, middle),
        [_, _, tail @ ..] => println!("arr[0] and arr[1] ignored, tail: {:?}", tail),
    }
    let [a, b, c, d, e] = arr;
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    println!();

    let c = Color::HSV {
        h: 1,
        s: 100,
        v: 25,
    };
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGB(RGB { r: 0, g, b }) => {
            println!("RGB: 0 {g} {b}");
        }
        Color::RGB(rgb) => {
            println!("RGB: {} {} {}", rgb.r, rgb.g, rgb.b);
        }
        Color::HSV { h, s, v } => {
            println!("HSV: {h} {s} {v}");
        }
        Color::CMYK(c, m, y, k) => {
            println!("CMYK: {c} {m} {y} {k}");
        }
    }
}
