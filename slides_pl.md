---
title: Rust
sub_title: Szybki przegląd i wprowadzenie
author: Mikołaj Juda
---

Przegląd
===

Rust jest kompilowanym, wieloparadygmatowym językiem ogólnego przeznaczenia
skupiającym się, między innymi, na bezpieczeństwie, niezawodności i wydajności.

<!-- column_layout: [3, 2] -->

<!-- column: 1 -->

![](rust_logo.png)

<!-- column: 0 -->

## ważne cechy:
- bezpieczeństwo pamięci bez potrzeby automatycznego odśmiecania
- zapobieganie wyścigom danych między wątkami
- silny system typów inspirowany językami funkcyjnymi (bazuje na nim obsługa błędów)
- możliwość programowania niskopoziomowego (wskaźniki, unsafe, inline assembly)
- interoperacyjność z językiem C (i nie tylko)
- makra (deklaratywne i proceduralne)
- wydajność porównywalna z językiem C
- użycie infrastruktury LLVM
- z kompilatorem dostarczany jest menadżer pakietów Cargo zintegrowany z repozytorium [crates.io](https://crates.io) (i inne narzędzia)

<!-- reset_layout -->
<!-- end_slide -->

Maskotka
---
Maskotką języka Rust jest krab Ferris.

![](ferris.png)

<!-- end_slide -->

Popularność
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

# Firmy i projekty używające języka Rust

- AWS
- Google
- Huawei
- Microsoft
- Facebook
- Cloudflare
- Dropbox
- Coursera
- Mozilla
- Discord
- Figma
- Npm
- SurrealDB
- 1Password
- Wire

i wiele innych

<!-- column: 1 -->

# Stack Overflow Development Survey

Przez ostatnie 8 lat (2016-2023) Rust
zajmował pierwsze miejsce jako najbardziej uwielbiany język programowania.

- 2023: 84.66%
- 2022: 86.73%
- 2021: 86.98%
- 2020: 86.1%
- 2019: 83.5%
- 2018: 78.9%
- 2017: 73.1%
- 2016: 79.1%

<!-- reset_layout -->
<!-- end_slide -->

Wprowadzenie
===

# Hello World
```rust
fn main() {
    println!("Hello, world! 🦀"); // println! is a macro
}
```

# Kompilacja

```
$ rustc hello_world.rs
$ ./hello_world
Hello, world! 🦀
```
W większości przypadków używanie narzędzia Cargo do budowania projektów jest preferowane od bezpośredniego wywoływania kompilatora.

<!-- end_slide -->

Typy proste
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

# Typy liczbowe

## Liczby całkowite
| **rozmiar**       | **ze znakiem** | **bez znaku** |
|-------------------|----------------|---------------|
| 8                 | `i8`           | `u8`          |
| 16                | `i16`          | `u16`         |
| 32                | `i32`          | `u32`         |
| 64                | `i64`          | `u64`         |
| 128               | `i128`         | `u128`        |
| rozmiar wskaźnika | `isize`        | `usize`       |

Liczby ze znakiem używają kodu uzupełnień do dwóch.

## Liczby zmiennoprzecinkowe
Typy `f32` oraz `f64` zgodne ze standardem IEEE 754-2008.

# Typ "nigdy" `!`
- nie ma możliwych wartości
- obecnie niestabilny, zamiast niego jest używany `Infallible` (pusty enum)

<!-- column: 1 -->

# Typ logiczny `bool`

# Typy tekstowe

## `char`
- zajmuje 4 bajty
- reprezentuje wartość skalarną Unikodu

## Wycinek napisu `str`
- `[u8]` z dodatkowym założeniem poprawności jako zakodowanie ciągu wartości skalarnych Unikodu w UTF-8
- zwykle używany poprzez typy wskaźnikowe np. `&str`

# Typ jednostkowy `()`
- pusta krotka
- zerowy rozmiar
- jedna możliwa wartość

<!-- reset_layout -->
<!-- end_slide -->

Zmienne
---

```rust
// static items have a precise memory location
static NUM: i64 = 123;
// constants don't have a memory location and are inlined
const PI: f64 = 3.141592;
fn takes_i64(_x: i64) {}
fn main() {
    let 𐐘: i32; // explicit type i32 (i32 is also the default for integers)
    let y = 10u8; // type inferred from literal u8
    let mut a = 1; // variables are immutable by default
    let b = 2;
    let _żółć: (); // underscore prefix suppresses unused variable warning
    a += y + 1; // type of a inferred here u8
    takes_i64(b); // type of b inferred here i64
    𐐘 = 5; // variables can be initialised after declaration
    println!("a = {}\n", a);
    let a = 0.5f64; // shadowing
    println!("𐐘 = {}", 𐐘);
    println!("y = {}", y);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("PI = {}", PI);
    println!("NUM = {}", NUM);
}
```

<!-- end_slide -->

Typy sekwencyjne
---

# Krotki `(T1, T2, T3, /*etc.*/)`
- lista wartości o różnych typach
- stała długość
- Przykłady:
    - ()
    - (u8,)
    - (i64, i64)
    - ((), u8, f32)
- pola nazywane używając kolejnych liczb całkowitych odpowiadających pozycji w liście typów: `0`, `1`, `2` itp.

# Tablice `[T; N]`
- lista długości `N` wartości o tym samym typie `T`
- sprawdznie poprawności dostępu do elementów tablicy na etapie kompilacji i podczes działania programu

# Wycinki `[T]`
- typ o dynamicznym rozmiarze reprezentujący "widok" na listę elementów typu `T`
- zwykle używany poprzez typy wskaźnikowe
- sprawdznie poprawności dostępu do elementów tablicy na etapie kompilacji i podczes działania programu

<!-- end_slide -->

Przepływ sterowania
---

# Bloki
```rust
let x = -2;
{
    let x = 10; // item declarations in blocks are scoped to the block
    println!("x is {}", x); // x is 10
    // default return value of a block is ()
}
println!("x is {}", x); // x is -2
let y = {
    let y = 2;
    y * y - x
    /* if the final expression is not followed by a semicolon
    it becomes a return value of the block
    */
};
println!("y is {}", y); // y is 6
```

<!-- end_slide -->

Przepływ sterowania
---

# `if`

```rust
let x = -2;
if x < 0 {
    println!("x is negative");
} else if x > 0 {
    println!("x is positive");
} else {
    println!("x is zero");
}
let y = if x < 0 { -x } else { x };
println!("y is {}", y);
```
```
x is negative
y is 2
```

<!-- end_slide -->

Przepływ sterowania
---

# `loop`
```rust
let mut a = 1;
let b = loop {
    a *= 2;
    if a > 10 {
        break a;
    }
};
println!("first power of 2 greater than 10 is {}", b);
```
```
first power of 2 greater than 10 is 16
```

<!-- end_slide -->

Przepływ sterowania
---

# `while`
```rust
let mut a = 1;
while a <= 3 {
    println!("a is {}", a);
    a += 1;
}
```
```
a is 1
a is 2
a is 3
```

<!-- end_slide -->

Przepływ sterowania
---

# `for`
```rust
for i in 0..5 {
    println!("{}", i);
}
let arr = ["dog", "cat", "horse"];
for animal in arr {
    println!("{}", animal);
}
```
```
0
1
2
3
4
dog
cat
horse
```

<!-- end_slide -->

Przepływ sterowania
---

# Etykiety

```rust
'outer: loop {
    println!("outer");
    loop {
        println!("inner");
        break 'outer;
    }
    println!("never printed");
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
```

<!-- end_slide -->

Przepływ sterowania
---

# `continue`
```rust
for i in 0..=5 {
    if i < 5 {
        continue;
    }
    println!("{}", i);
}
```
```
5
```

<!-- end_slide -->

Typy definiowane przez użytkownika
---

# Struktury

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

## Struktury z nazwanymi polami
```rust
struct StructName {
    field1: T1,
    field2: T2,
    field3: T3,
    // etc.
}
```
podobne do struktur w C

<!-- column: 1 -->

## Struktury krotkowe
```rust
struct StructName(T1, T2, T3, /*etc.*/);
```
podobne do krotek

## Struktury jednostkowe
```rust
struct StructName;
```
podobne do `()`

<!-- reset_layout -->
<!-- end_slide -->

Typy definiowane przez użytkownika
---

# Typy wyliczeniowe
```rust
enum EnumName {
    Variant1,       // implicit discriminant 0
    Variant2 = 123, // explicit disscriminant 123
    Variant3,       // implicit discriminant 124
    // etc.
}

```
podobne do typów wyliczniowych w C
```rust
enum NumName {
    Variant1,
    Variant2(bool, u8),          // optional tuple constructor
    Variant3 { x: f32, y: f32 }, // optional struct constructor
    Variant4(i32),
    // etc.
}
```
podobne do tzw. rekordu z wariantami

<!-- end_slide -->

Typy definiowane przez użytkownika
---

# Unie

```rust
union StructName {
    field1: T1,
    field2: T2,
    field3: T3,
    // etc.
}
```
- podobne do unii w C
- jak struktury, ale pola dzielą pamięć
- dostęp do elementów jest niebezpieczny
- pewne ograniczenia typów pól

<!-- end_slide -->

Dopasowanie do wzorca
---

```rust
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
```
```
small prime
b is 1
```

<!-- end_slide -->

Dopasowanie do wzorca
---

```rust
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
fn main() {
    let c = RGB { r: 64, g: 0, b: 0 };
    match c {
        RGB { r: 0, g: 0, b: 0 } => println!("black"),
        RGB { r: r @ 0..=80, g: 0, b: 0, } => println!("some sort of dark red {r}"),
        RGB { b: 255, .. } => println!("something with maximum blue"),
        RGB { r, g, b } => println!("r: {}, g: {}, b: {}", r, g, b),
    }
    let RGB { r: red, g: green, b: blue, } = c;
    println!("red: {}, green: {}, blue: {}", red, green, blue);
}
```
```
some sort of dark red 64
red: 64, green: 0, blue: 0
```

<!-- end_slide -->

Dopasowanie do wzorca
---

```rust
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
```
```
ends with a letter sigma
a: 10, c: false, d: 0.1, e: Σ
```

<!-- end_slide -->

Dopasowanie do wzorca
---

```rust
let arr = [1, 2, 3, 4, 5];
match arr {
    [0, .., last] => println!("arr[0]: 0, last: {}", last),
    [first, middle @ .., 5] => println!("arr[0]: {}, middle: {:?}", first, middle),
    [_, _, tail @ ..] => println!("arr[0] and arr[1] ignored, tail: {:?}", first, tail),
}
let [a, b, c, d, e] = arr;
println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);
```
```
arr[0]: 1, middle: [2, 3, 4]
a: 1, b: 2, c: 3, d: 4, e: 5
```

<!-- end_slide -->

Dopasowanie do wzorca
---

<!-- column_layout: [1, 2] -->

<!-- column: 0 -->
```rust
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
enum Color {
    Red,
    Green,
    Blue,
    RGB(RGB),
    HSV{h: u8, s: u8, v: u8},
    CMYK(u8, u8, u8, u8),
}
```
```
HSV: 1 100 25
```

<!-- column: 1 -->

```rust
let c = Color::HSV { h: 1, s: 100, v: 25 };
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
```

<!-- reset_layout -->
<!-- end_slide -->

Funkcje
---

```rust
fn function_name(param1: T1, param2: T2, /*etc.*/) -> ReturnType {
    // stuff
    expression_of_type_ReturnType
}

fn function_without_return(param1: T1, param2: T2, /*etc.*/) {} // default return type is ()
```

```rust
fn is_nonnegative(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    true // return true; is also possible
}
```
<!-- end_slide -->

Typy wskaźnikowe
---

# Referencje
Referencje to wskaźniki do pamięci będącej własnością innej wartości.
Borrow checker zapewnia poprawność wszystkich referencji.

## Referencje współdzielone `&T`
- uniemożliwiają bezpośrednią modyfikację wskazywanej wartości
- mogą być kopiowane

## Referencje mutowalne `&mut T`
- umożliwia bezpośrednią modyfikację wskazywanej wartości
- w jednym momencie może istnieć tylko jedna referencja mutowalna do danej wartości

# Surowe wskaźniki `*const T` i `*mut T`
- brak żadnych gwarancji poprawności i bezpieczeństwa
- dereferencja surowego wskaźnika jest niebezpieczna

Referencje i wskaźniki na typy o dynamicznym rozmiarze stają się wskaźnikami szerokimi (mają dodatkową informację o rozmiarze).

<!-- end_slide -->

Niektóre typy z biblioteki standardowej
---

# `Vec`
kontener sąsiadującej pamięci o zmiennym rozmiarze na dane tego samego typu
```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
println!("{:?}", vec);
```

# `String`
napis w kodowaniu UTF-8 o zmiennym rozmiarze
```rust
let mut string: String = "Hello".to_string();
string.push_str(" world");
string.push('!');
println!("{}", string);
```

<!-- end_slide -->

Własność
---

Zamiast zmuszać programistę do ręcznego alokowania i zwalniania pamięci lub polegać na odśmiecaniu przez garbage collector,
Rust do zarządzania pamięcią wykorzystuje system własności z zestawem reguł sprawdzanych przez kompilator.

# Zasady systemu własności
- Każda wartość ma właściciela.
- W danym momencie może istnieć tylko jeden właściciel (dla danej wartości).
- Kiedy właściciel wychodzi z zasięgu, posiadana przez niego wartość zostaje zwolniona.

```rust
{ // x is not in scope yet
    let x = 5; // x enters the scope here
} // x goes out of scope here
println!("x is {}", x); // compiler error: cannot find value `x` in this scope
```
Rust używa wzorca RAII.
```rust
{
    let s = "hello".to_string(); // s is valid form here (memory allocated)
} // drop method called (destructor) releasing heap memory
```

<!-- end_slide -->

Semantyka przenoszenia
---

```rust
let s1 = "hello".to_string();
let s2 = s1; // assignment operator transfers ownership (unless type implements Copy trait)
// s1 is no longer valid, destructor will not be called
println!("{s2}");
println!("{s1}"); // compiler error: borrow of moved value: `s1`
```
```rust
fn print_string(s: String) {
    println!("{s}");
}

fn main() {
    let s1 = "hello".to_string();
    print_string(s1); // function parameter takes ownership
    print_string(s1); // compiler error: use of moved value: `s1`
}
```

<!-- end_slide -->

Semantyka przenoszenia
---

```rust
fn make_string() -> String {
    "hello".to_string()
}

fn main() {
    let s = make_string();
    println!("{s}");
}
```
```rust
fn print_and_return_string(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let s = "hello".to_string();
    // ownership transfered to function and then back to new s
    let s = print_and_return_string(s);
    print_and_return_string(s);
}
```

<!-- end_slide -->

Pożyczanie
---

Operacje na wartościach można wykonywać bez przejmowania nad nimi własności używając referencji.
Tworzenie referencji nazywa się *pożyczaniem*.

# Zasady dotyczące referencji
- W każdym momencie może istnieć *albo* dowolna ilość referencji współdzielonych
*albo* jedna referencja mutowalna (dla danej wartości).
- Referencje zawsze muszą być prawidłowe

<!-- end_slide -->

Pożyczanie
---

```rust
fn print_string(s: &String) {
    println!("{s}");
} // s goes out of scope, but it doesn't have ownership of the string, so it's not dropped

fn main() {
    let s = "hello".to_string();
    print_string(&s); // we pass a reference instead of the string itself
    print_string(&s); // s is valid here
}
```

```rust
fn change_string(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s1 = "hello".to_string();
    println!("{}", s1); // hello
    change_string(&mut s1);
    println!("{}", s1); // hello!
}
```

<!-- end_slide -->

Pożyczanie
---

```rust
let s = "hello".to_string();
let r1 = &s;
let r2 = &s;
println!("{r1}");
println!("{r2}");
```

```rust
let mut s = "hello".to_string();
let r1 = &s;
let r2 = &mut s; /* compiler error:
cannot borrow `s` as mutable because it is also borrowed as immutable */
println!("{r1}");
println!("{r2}");

```

```rust
let mut s1 = "hello".to_string();
let r1 = &mut s1;
let r2 = &mut s1; // compiler error: cannot borrow `s1` as mutable more than once at a time
println!("{}", r1);
println!("{}", r2);
```

<!-- end_slide -->

Pożyczanie
---

```rust
fn change_string(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s1 = "hello".to_string();
    let r1 = &mut s1;
    // compiler error: cannot borrow `s1` as mutable more than once at a time
    let r2 = &mut s1;
    change_string(r1);
    change_string(r2);
    println!("{}", s1);
}
```

<!-- end_slide -->

Pożyczanie
---

```rust
// this will not compile
fn make_string_reference() -> &String {
    let s = "hello".to_string();
    &s // you can't return a reference to a value scoped to the function
} // s is dropped here so the reference would be invalid

fn main() {
    let s = make_string_reference();
}
```

<!-- end_slide -->

Pożyczanie
---

# Wycinki

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust
fn print_str(s: &str) {
    println!("{}", s);
}
fn first_four_letters(s: &String) -> &str {
    &s[..4] /* caution:
    byte indexing, use with ascii only*/
}
fn square_slice(s: &mut [i32]) {
    for i in s {
        *i = *i * *i;
    }
}
fn main() {
    let s = "Hello, world!".to_string();
    println!("{}", first_four_letters(&s));
    print_str(&s[7..=11]);

    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
    square_slice(&mut arr[3..]); /* caution:
    byte indexing, use with ascii only*/
    println!("{:?}", arr);
}
```

<!-- column: 1 -->

```
Hell
world
[1, 2, 3, 16, 25, 36, 49, 64]
```

<!-- reset_layout -->
<!-- end_slide -->

Metody i funkcje powiązane
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
struct Human {
    name: String,
    age: u32,
}

impl Human {
    // associated function
    fn age_difference(h1: &Self, h2: &Self) -> i64 {
        h1.age as i64 - h2.age as i64
    }
}
```

<!-- column: 1 -->

```rust
fn main() {
    let human1 = Human {
        name: String::from("Steve"),
        age: 2,
    };
    let human2 = Human {
        name: String::from("John"),
        age: 123,
    };
    println!(
        "age difference is {}",
        Human::age_difference(&human1, &human2)
    );
}
```
```
age difference is -121
```

<!-- reset_layout -->
<!-- end_slide -->

Metody i funkcje powiązane
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
struct Human {
    name: String,
    age: u32,
}

impl Human {
    // method (also associated function)
    // &self is an alias for self: &Self
    fn say_name(&self) {
        println!("My name is {}", self.name);
    }

    // &mut self is an alias for self: &mut Self
    fn birthday(&mut self) {
        self.age += 1;
    }

    // self is an alias for self: Self
    fn die(self) {
        println!("{} dies", self.name);
    } // self goes out of scope and is dropped
}
```

<!-- column: 1 -->

```rust
fn main() {
    let mut human = Human {
        name: String::from("John"),
        age: 30,
    };
    human.say_name();
    human.birthday();
    println!("{} is {}", human.name, human.age);
    human.die();
    // human is not valid anymore
}
```
```
My name is John
John is 31 years old
John dies
```

<!-- reset_layout -->
<!-- end_slide -->

Traits
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
struct Dog;
struct Cat;
struct Human {
    name: String,
}
trait Animal {
    fn speak(&self); // no default implementation
    // default implementation
    fn is_alive(&self) -> bool {
        true
    }
}
impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("meow!");
    }
}
```

<!-- column: 1 -->

```rust
impl Animal for Human {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
fn main() {
    let dog = Dog;
    let cat = Cat;
    let person = Human {
        name: "John".to_string(),
    };
    dog.speak();
    cat.speak();
    person.speak();
    println!("Is the cat alive? {}", cat.is_alive());
}
```
```
woof!
meow!
Hello, my name is John
Is the cat alive? true
```

<!-- reset_layout -->
<!-- end_slide -->

Traits
---

# Niektóre cechy wbudowane
- Debug - umożliwia formatowanie w kontekście debugowania
- Display - umożliwia "ładne" formatowanie
- Clone - umożliwia klonowanie (metoda `clone`)
- Copy - typ jest kopiowany zamiast przenoszenia
- Drop - destruktor (metoda `drop`)
- Deref i DerefMut - przeładowania operatora `*`
- Default - domyślne wartości (funkcja `default`)
- Eq - porównanie będące relacją równoważności
- PartialEq - jak Eq, ale porównania nie muszą być zwrotne
- Ord - porządek liniowy
- PartialOrd - porządek częściowy
- Hash - umożliwia hashowanie
- Send - wartość może być wysyłana między wątkami
- Sync - wartość może być dzielona między wątkami
- Sized - rozmiar znany na etapie kompilacji

<!-- end_slide -->

Traits
---

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Human {
    name: String,
    age: u32,
}
fn main() {
    let default_human = Human::default();
    println!("default human: {:?}", default_human);
    let human = Human {
        name: "John".to_string(),
        age: 40,
    };
    let cloned_human = human.clone();
    println!("human: {:?}", human);
    println!("is human equal to default_human? {}", human==default_human);
    println!("is human equal to cloned_human? {}", human==cloned_human);
}
```
```
default human: Human { name: "", age: 0 }
human: Human { name: "John", age: 40 }
is human equal to default_human? false
is human equal to cloned_human? true
```

<!-- end_slide -->

Uogólnienia
---

```rust
struct Point<T> { x: T, y: T, }
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
    fn y(&self) -> &T { &self.y }
}
impl Point<String> {
    fn special_x(&self) -> String {
        format!("idk why Strings but here you go: {}", self.x)
    }
}
fn main() {
    let p1: Point<i16> = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.8, y: 10.2 };
    let p3 = Point { x: "hello".to_string(), y: "world".to_string() };
    println!("p1.x(): {}, p2.y(): {}, p3.x(): {}", p1.x(), p2.y(), p3.x());
    println!("p3.special_x(): {}", p3.special_x());
}
```
```
p1.x(): 5, p2.y(): 10.2, p3.x(): hello
p3.special_x(): idk why Strings but here you go: hello
```

<!-- end_slide -->

Uogólnienia
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust
fn duplicate<T: Clone>(x: &T) -> (T, T) {
    (x.clone(), x.clone())
}
// syntactic sugar for
// fn generic_print<T: std::fmt::Display + ?Sized>(x: &T) {
fn generic_print(x: &(impl std::fmt::Display + ?Sized)) {
    println!("generically printed: {}", x);
}
fn square<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}
fn main() {
    let s = "hello".to_string();
    let (s1, s2) = duplicate(&s);
    println!("s1: {}, s2: {}", s1, s2);
    generic_print(&123);
    generic_print("hello");
    println!("square(8): {}", square(8i8));
    println!("square(2.5): {}", square(2.5f64));
}
```

<!-- column: 1 -->

```
s1: hello, s2: hello
generically printed: 123
generically printed: hello
square(8): 64
square(2.5): 6.25
```

<!-- reset_layout -->
<!-- end_slide -->

Uogólnienia
---

```rust
trait Square<T> {
    fn square(self) -> T;
}

impl<T> Square<T> for T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn square(self) -> T {
        self * self
    }
}

fn main() {
    println!("{}", 2u8.square());
    println!("{}", 16.5f32.square());
}
```

<!-- end_slide -->

Czasy życia
---

Czasy życia to konstrukt kompilatora używany do sprawdzania poprawności pożyczeń.

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
fn main() {
    let x = 5; // lifetime of x starts here
    {
        let r1 = &x; // lifetime for r1 starts here

        println!("r1: {}", r1);
    } // lifetime of r1 ends here
    {
        let r2 = &x; // lifetime for r2 starts here

        println!("r2: {}", r2);
    } // lifetime of r2 ends here
} // lifetime of x ends here
```
<!-- column: 1 -->

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x; /* compiler error:
        `x` does not live long enough*/
    }
    println!("r: {}", r);
}
```
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

<!-- reset_layout -->
<!-- end_slide -->

Czasy życia
---

# Czasy życia w wywołaniach funkcji

```rust
fn longest_str(x: &str, y: &str) -> &str { /*compiler error:
    expected named lifetime parameter*/
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "xyz";
    let s2 = String::from("abcd");
    let result = longest_str(s1, s2.as_str());
    println!("The longest string is {}", result);
}
```
<!-- end_slide -->

Czasy życia
---

```rust
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "xyz".to_string();
    let s2 = "abcd".to_string();
    let result = longest_str(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);
}
```
Sygnatura funkcji oznacza teraz, że dla jakiegoś czasu życia `'a` funkcja `longest_str` pożycza dwa wycinki napisów,
które żyją *przynajmniej* tak długo jak `'a` i zwraca referencję do wycinka napisu,
który także żyje *przynajmniej* tak długo jak `'a`.
Kompilator sam znajdzie taki czas życia.
W tym przypadku oznacza to, że wartość wskazywana przez zwracaną referencję,
żyje tak długo jak krócej żyjąca spośród wartości wskazywanych przez referencje w parametrach.

<!-- end_slide -->

Czasy życia
---

# Czasy życia w strukturach danych

```rust
struct Human {
    name: String,
}

struct Pet<'a> {
    name: String,
    owner: &'a Human,
}

fn main() {
    let human = Human {
        name: "John".to_string(),
    };
    let pet = Pet {
        name: "Steve".to_string(),
        owner: &human,
    };
    println!("{}'s owner name is {}", pet.name, pet.owner.name);
}
```

<!-- end_slide -->

Czasy życia
---

# `'static`

```rust
static NUM: u8 = 6;

// T doesn't contain non-static references
fn print_static<T: std::fmt::Display + 'static>(x: T) {
    println!("{}", x);
}

fn main() {
    let s: &'static str = "hello"; // string literals have 'static lifetime
    print_static(s); // hello
    print_static(&NUM); // 6
    let x = 5;
    print_static(x); // 5
    print_static(&x); // compiler error: `x` does not live long enough
}
```

<!-- end_slide -->

Cargo
---

