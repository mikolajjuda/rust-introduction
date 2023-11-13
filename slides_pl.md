---
title: Rust
sub_title: Szybki przegląd i wprowadzenie
author: Mikołaj Juda
theme:
  override:
   footer:
    style: template
    right: "{current_slide} / {total_slides}"
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
===
Maskotką języka Rust jest krab Ferris.

![](ferris.png)

<!-- end_slide -->

Popularność
===

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

# Firmy i projekty używające języka Rust

- Mozilla
- Google
- AWS
- Microsoft
- Huawei
- Facebook
- Cloudflare
- Dropbox
- Coursera
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

# Instalacja

Rekomendowanym sposobem instalacji narzędzi potrzebnych do korzystania z języka Rust
jest wykorzystanie narzędzia `rustup` ([rustup](https://rustup.rs/)).
Zajmuje się ono instalacją zestawów narzędzi w skład których wchodzą m.in.
- `rustc` - kompilator
- `rustdoc` - generator dokumentacji
- `cargo` - menadżer pakietów i narzędzie do budowania
- `clippy` - linter
- `rustfmt` - formater
- `rust-std` - biblioteka standardowa

<!-- end_slide -->

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
===

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
===

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
    𐐘 = 5; // variables can be initialized after declaration
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
===

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
- sprawdzanie poprawności dostępu do elementów tablicy na etapie kompilacji i podczas działania programu

# Wycinki `[T]`
- typ o dynamicznym rozmiarze reprezentujący "widok" na listę elementów typu `T`
- zwykle używany poprzez typy wskaźnikowe
- sprawdzanie poprawności dostępu do elementów tablicy na etapie kompilacji i podczas działania programu

<!-- end_slide -->

Przepływ sterowania
===

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
===

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

# Typy wyliczeniowe
```rust
enum EnumName {
    Variant1,       // implicit discriminant 0
    Variant2 = 123, // explicit discriminant 123
    Variant3,       // implicit discriminant 124
    // etc.
}

```
podobne do typów wyliczeniowych w C
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
===

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

# `if let`

```rust
enum Animal {
    Dog(String),
    Cat { remaining_lives: u8 },
    Bird,
}

fn main() {
    let animal = Animal::Dog(String::from("Steve"));
    if let Animal::Dog(name) = animal {
        println!("Found a dog named {}", name);
    } else {
        println!("Some other animal");
    }
}
```

# `while let`

Możemy się domyślić jak działa.

<!-- end_slide -->

Funkcje
===

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
===

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
===

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
let mut string = String::from("hello");
string.push_str(" world");
string.push('!');
println!("{}", string);
```

<!-- end_slide -->

Własność
===

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
    let s: String = "hello".to_owned(); // s is valid form here (memory allocated)
} // drop method called (destructor) releasing heap memory
```

<!-- end_slide -->

Semantyka przenoszenia
===

```rust
let s1 = "hello".to_owned();
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
    let s1 = "hello".to_owned();
    print_string(s1); // function parameter takes ownership
    print_string(s1); // compiler error: use of moved value: `s1`
}
```

<!-- end_slide -->

```rust
fn make_string() -> String {
    "hello".to_owned()
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
    let s = "hello".to_owned();
    // ownership transferred to function and then back to new s
    let s = print_and_return_string(s);
    print_and_return_string(s);
}
```

<!-- end_slide -->

Pożyczanie
===

Operacje na wartościach można wykonywać bez przejmowania nad nimi własności używając referencji.
Tworzenie referencji nazywa się *pożyczaniem*.

# Zasady dotyczące referencji
- W każdym momencie może istnieć *albo* dowolna ilość referencji współdzielonych
*albo* jedna referencja mutowalna (dla danej wartości).
- Referencje zawsze muszą być prawidłowe

<!-- end_slide -->

```rust
fn print_string(s: &String) {
    println!("{s}");
} // s goes out of scope, but it doesn't have ownership of the string, so it's not dropped

fn main() {
    let s = "hello".to_owned();
    print_string(&s); // we pass a reference instead of the string itself
    print_string(&s); // s is valid here
}
```

```rust
fn change_string(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s1 = "hello".to_owned();
    println!("{}", s1); // hello
    change_string(&mut s1);
    println!("{}", s1); // hello!
}
```

<!-- end_slide -->

```rust
let s = "hello".to_owned();
let r1 = &s;
let r2 = &s;
println!("{r1}");
println!("{r2}");
```

```rust
let mut s = "hello".to_owned();
let r1 = &s;
let r2 = &mut s; /* compiler error:
cannot borrow `s` as mutable because it is also borrowed as immutable */
println!("{r1}");
println!("{r2}");

```

```rust
let mut s1 = "hello".to_owned();
let r1 = &mut s1;
let r2 = &mut s1; // compiler error: cannot borrow `s1` as mutable more than once at a time
println!("{}", r1);
println!("{}", r2);
```

<!-- end_slide -->

```rust
// this will not compile
fn make_string_reference() -> &String {
    let s = "hello".to_owned();
    &s // you can't return a reference to a value scoped to the function
} // s is dropped here so the reference would be invalid

fn main() {
    let s = make_string_reference();
}
```

<!-- end_slide -->

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
    let s = "Hello, world!".to_owned();
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
===

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
===

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
        name: "John".to_owned(),
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

# Niektóre cechy wbudowane
- Debug - umożliwia formatowanie w kontekście debugowania
- Display - umożliwia "ładne" formatowanie
- Clone - umożliwia klonowanie (metoda `clone`)
- Copy - typ jest kopiowany zamiast przenoszenia
- Drop - destruktor (metoda `drop`)
- ToOwned - tworzy wartość posiadaną z wartości pożyczonej
- Deref i DerefMut - przeładowania operatora `*`
- Default - domyślne wartości (funkcja `default`)
- Eq - porównanie będące relacją równoważności
- PartialEq - jak Eq, ale porównania nie muszą być zwrotne
- Ord - porządek liniowy
- PartialOrd - porządek częściowy
- Hash - umożliwia haszowanie
- Send - wartość może być wysyłana między wątkami
- Sync - wartość może być dzielona między wątkami
- Sized - rozmiar znany na etapie kompilacji

<!-- end_slide -->

Wyprowadzanie cech
===

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
        name: "John".to_owned(),
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

Supertraits
===

```rust
struct Human {
    name: String,
}
impl std::fmt::Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

trait Speaker: std::fmt::Display {
    fn speak(&self) {
        println!("{} says hello", self);
    }
}
impl Speaker for Human {}

fn main(){
    let human = Human { name: "John".to_owned() };
    human.speak();
}
```

<!-- end_slide -->

Uogólnienia
===

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
    let p3 = Point { x: "hello".to_owned(), y: "world".to_owned() };
    println!("p1.x(): {}, p2.y(): {}, p3.x(): {}", p1.x(), p2.y(), p3.x());
    println!("p3.special_x(): {}", p3.special_x());
}
```
```
p1.x(): 5, p2.y(): 10.2, p3.x(): hello
p3.special_x(): idk why Strings but here you go: hello
```

<!-- end_slide -->

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
    let s = "hello".to_owned();
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

Mądre wskaźniki
===

Struktury danych, które zachowują się jak typy wskaźnikowe, ale mają dodatkowe możliwości.

# Niektóre wbudowane mądre wskaźniki

## `Box<T>`
Wskaźnik, który jest właścicielem wartości na stercie.
Podobny do `unique_ptr` w C++.

## `Rc<T>`
Wskaźnik liczący referencje.
Podobny do `shared_ptr` w C++.

## `Ref<T>` i `RefMut<T>`
Wskaźniki pozyskiwane z `RefCell<T>`.
Pilnują zasad pożyczania podczas działania programu zamiast na etapie kompilacji.

<!-- end_slide -->

Polimorfizm dynamiczny
===

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust
struct Human { name: String, }
struct Dog;
trait Speaker {
    fn speak(&self);
}
impl Speaker for Human {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
impl Speaker for Dog {
    fn speak(&self) {  println!("Woof!"); }
}
fn main() {
    let v: Vec<Box<dyn Speaker>> = vec![
        Box::new(Human { name: "John".to_owned() }),
        Box::new(Dog),
    ];
    for s in v.iter() {
        s.speak();
    }
}
```

<!-- column: 1 -->

```
Hello, my name is John
Woof!
```

<!-- reset_layout -->
<!-- end_slide -->

Czasy życia
===

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

```rust
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "xyz".to_owned();
    let s2 = "abcd".to_owned();
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
        name: "John".to_owned(),
    };
    let pet = Pet {
        name: "Steve".to_owned(),
        owner: &human,
    };
    println!("{}'s owner name is {}", pet.name, pet.owner.name);
}
```

<!-- end_slide -->

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

Mutowalność wnętrza
===

```rust
use std::cell::{Cell, RefCell};
struct Human {
    name: RefCell<String>,
}
struct Pet {
    age: Cell<u8>,
}
fn main() {
    let human = Human {
        name: RefCell::new("John".to_owned()),
    };
    let human_ref = &human;
    let pet = Pet { age: Cell::new(10) };
    let pet_ref = &pet;

    // we can't borrow a value behind a Cell
    let tmp_pet_age = pet_ref.age.get(); // we can get a value
    println!("pet age: {}", tmp_pet_age);
    pet_ref.age.set(tmp_pet_age + 1); // we can set a new value
    println!("pet age: {}", pet_ref.age.get());

    println!("human name: {}", human_ref.name.borrow());
    // we can mutably borrow interior of a RefCell behind an immutable reference
    human_ref.name.borrow_mut().push_str("athan");
    println!("human name: {}", human_ref.name.borrow());
}
```

<!-- end_slide -->

Organizacja projektu
===

- Pakiet - zbiór skrzynek
- Crate - jednostka kompilacji; drzewo modułów, które kompiluje się do pliku wykonywalnego lub biblioteki
- Moduł - jednostka organizacyjna kodu umożliwiająca kontrolę prywatności
- Ścieżka - sposób odwoływania się do elementów w drzewie modułów

# Tworzenie projektu
```
$ cargo new a
$ cargo new b --lib
$ tree
.
├── a
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── b
    ├── Cargo.toml
    └── src
        └── lib.rs
```

<!-- end_slide -->

# Uruchamianie projektu

```
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```
- `cargo build` - budowanie projektu
- `cargo run` - budowanie i uruchamianie

```
$ cargo run
Compiling my_project v0.1.0 (/path/to/my_project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/my_project`
Hello, world!
```

Flaga `--release` włącza pełną optymalizację.

<!-- end_slide -->

# Zależności

Zależności są opisywane w sekcjach `[dependencies]` pliku `Cargo.toml`.

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
random = "*"
image = { version = "0.24.7", default-features = false, features = ["png"] }
```

<!-- end_slide -->

# Moduły

```rust
mod outer {
    fn private() {
        println!("outer::private");
    }
    pub fn public() {
        println!("outer::public");
    }
    mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}
fn main() {
    outer::public();
}
```

<!-- end_slide -->

# Widoczność i prywatność

```rust
mod outer{
    pub mod inner {
        #[derive(Default)]
        pub struct MyStruct{
            pub my_public_field: u8,
            pub(super) my_less_public_field: u8,
            my_private_field: u8,
        }
    }
    fn outer_function(x: inner::MyStruct){
        println!("{}", x.my_public_field);
        println!("{}", x.my_less_public_field);
        println!("{}", x.my_private_field); /* error:
        field `my_private_field` of struct `MyStruct` is private */
    }
}

fn main() {
    let s = outer::inner::MyStruct::default();
    println!("{}", s.my_public_field);
    println!("{}", s.my_less_public_field); /* error:
    field `my_less_public_field` of struct `MyStruct` is private */
    println!("{}", s.my_private_field); /* error:
    field `my_private_field` of struct `MyStruct` is private */
}
```
<!-- end_slide -->

# Ścieżki

- względne
    - `foo` lub `self::foo` odnosi się do `foo` w obecnym module
    - `super::foo` odnosi się do `foo` w rodzicu
- bezwzgledne
    - `crate::foo` odnosi się do `foo` w korzeniu obecnej skrzynki
    - `bar::foo` odnosi się do `foo` w skrzynce `bar`

# `use`

Słowo kluczowe `use` definiuje lokalne przypisania dla symboli z innych modułów.

```rust
use std::fmt::Display as Disp;
use std::collections::{HashMap, HashSet};
use random::Source;
```
<!-- end_slide -->

Kompilacja warunkowa
===

```rust
#[cfg(not(feature = "demo_feature"))]
fn some_function() {
    println!("hello");
}

#[cfg(feature = "demo_feature")]
fn some_function() {
    #[cfg(target_os = "linux")]
    println!("linux");
    #[cfg(target_os = "windows")]
    println!("windows");
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    println!("something else");
}

fn main() {
    some_function();
}
```

<!-- end_slide -->

Obsługa błędów
===

W języku Rust błędy są obsługiwane w jawnym przepływie sterowania.
Funkcje, które mogą zakończyć się niepowodzeniem mają to zapisane w wartości zwracanej.

W przypadku nieoczekiwanych błędów, których nie da się obsłużyć program panikuje (`panic!()`).
W przypadku paniki zależnie od ustawień kompilatora stos zostaje zwijany (domyślna opcja, destruktory są wywoływane, można złapać)
lub program zostaje natychmiastowo przerwany (abort).

Rust domyślnie udostępnia typy służące ustrukturyzowanej obsłudze błędów:
<!-- column_layout: [1, 1] -->

<!-- column: 0 -->


```rust
// core::option::Option
pub enum Option<T> {
    Some(T),
    None,
}
```

<!-- column: 1 -->

```rust
// core::result::Result
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!-- reset_layout -->
<!-- end_slide -->

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<i32, String>::new();
    map.insert(123, "a".to_owned());
    map.insert(321, "b".to_owned());
    match map.get(&666) {
        Some(n) => println!("{}", n),
        None => println!("No such key")
    }
    let x: &String = map.get(&123).unwrap(); // panics if None
    println!("{}", x);
    println!("{}", map[&321]); // panics if no such key
    println!("{}", map.get(&1000).expect("oops")); // panics on None with a message
}
```
```
No such key
a
b
thread 'main' panicked at 'oops', src/bin/hashmap_some.rs:16:35
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!-- end_slide -->

```rust
fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed: &str = input.trim();
            match trimmed.parse::<i8>() {
                Ok(i) => println!("your integer input: {}", i),
                Err(e) => println!("parsing error: {}", e),
            }
        }
        Err(_) => panic!("error: unable to read line from stdin"),
    }
}
```

<!-- end_slide -->

```rust
fn print_str_as_i32(s: &str) -> Result<(), Box<dyn std::error::Error>> {
    let x: i32 = s.parse()?;
    println!("{}", x);
    Ok(())
}

fn main(){
    let _ = print_str_as_i32("10");
}
```
<!-- end_slide -->

Konwersja między typami
===

# Rzutowanie

```rust
let decimal = 65.99999f64;
let integer = decimal as u8;
let character = integer as char;
println!("{} -> {} -> {}", decimal, integer, character);
println!("-1i8 as u8 {}", -1i8 as u8);
println!("1000u32 as u8 {}", 1000u32 as u8);
println!(" 232i32 as i8 {}", 232i32 as i8);
println!("-10023f32 as u8 {}", -1023f32 as i8);
println!("NAN f32 as u8 {}", f32::NAN as u8);
```
```
65.99999 -> 65 -> A
-1i8 as u8 255
1000u32 as u8 232
 232i32 as i8 -24
-10023f32 as u8 -128
NAN f32 as u8 0
```
<!-- end_slide -->

# `From` i `Into`

```rust
#[derive(Debug, Clone)]
struct Human{ name: String, age: u32 }

#[derive(Debug)]
struct Dog{ name: String, age: u32 }

impl From<Human> for Dog {
    fn from(human: Human) -> Self {
        Dog { name: human.name, age: human.age }
    }
}
fn main() {
    let human = Human { name: String::from("John"), age: 30 };
    println!("human: {:?}", human);
    let dog = Dog::from(human.clone());
    let dog2: Dog = human.into(); // Into trait implemented for Human automatically
    println!("dog: {:?}", dog);
    println!("dog2: {:?}", dog2);
}
```
```
human: Human { name: "John", age: 30 }
dog: Dog { name: "John", age: 30 }
dog2: Dog { name: "John", age: 30 }
```
<!-- end_slide -->

# `TryFrom` i `TryInto`

```rust
let x: i128 = 5637245724782828626582567367;
match TryInto::<i64>::try_into(x) {
    Ok(i) => println!("i64 value is {}", i),
    Err(e) => println!("Error: {}", e),
}

let x: u16 = 2i8.try_into().unwrap();
println!("u16 value is {}", x);
```
```
Error: out of range integral type conversion attempted
u16 value is 2
```
<!-- end_slide -->

Przepełnienie przy operacjach arytmetycznych
===

```rust
println!("225u8 + 1 = {}", 255u8 + 1); // compile error in debug mode; 0 in release mode
let x255: u8 = "255".parse().unwrap();
let x0: u8 = "0".parse().unwrap();
println!("{} + 1 = {}", x255, x255 + 1); // panic in debug mode; 0 in release mode
println!("{} - 1 = {}", x0, x0 - 1); // panic in debug mode; 255 in release mode
println!("{} wrapping_add 1 = {}", x255, x255.wrapping_add(1)); // 0
println!("{} wrapping_sub 1 = {}", x0, x0.wrapping_sub(1)); // 255
println!("{} saturating_add 1 = {}", x255, x255.saturating_add(1)); // 255
println!("{} overflowing_add 1 = {:?}", x255, x255.overflowing_add(1)); // (0, true)
println!("{} checked_add 1 = {:?}", x255, x255.checked_add(1)); // None
let wrapping_255 = std::num::Wrapping(255u8);
let wrapping_1 = std::num::Wrapping(1u8);
let wrapping_0 = std::num::Wrapping(0u8);
println!("Wrapping({}) + Wrapping(1) = {}", wrapping_255, wrapping_255 + wrapping_1); // 0
println!("Wrapping({}) - Wrapping(1) = {}", wrapping_0, wrapping_0 - wrapping_1); // 255
```
<!-- end_slide -->

Closures
===

<!-- column_layout: [4, 2] -->

<!-- column: 0 -->

```rust
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
    run_mut_closure(&mut |x| { b += x; });
    println!("b after calling closure: {}", b);
    let s = "hello".to_owned();
    let owns_string = move || { // s is moved into the closure
        println!("string: {}", s);
        s
    };
    // s is invalid here
    println!("before closure");
    run_once_closure(owns_string);
    println!("after closure");
}
```

<!-- column: 1 -->

```
closure with parameter 6 returned 7
closure with parameter 6 returned 26
b before calling closure: 10
b after calling closure: 22
before closure
string: hello
we own string: hello
after closure
```

<!-- end_slide -->

Iteratory
===

```rust
let v = vec![1, 2, 3];
let mut iter = v.into_iter(); // into_iter() method from IntoIterator trait
// methods iter() and iter_mut() iterate over &T and &mut T
println!("{:?}", iter.next()); // next() method from Iterator trait
println!("{:?}", iter.next());
println!("{:?}", iter.next());
println!("{:?}", iter.next());
// into_iter() takes ownership; v can't be used here
```
```
Some(1)
Some(2)
Some(3)
None
```

<!-- column_layout: [1, 1, 1] -->

<!-- column: 0 -->

```rust
let v = vec![1, 2, 3];
for i in v {
    println!("{:?}", i);
}
```

<!-- column: 1 -->

```
1
2
3
```

<!-- column: 2 -->

```rust
let v = vec![1, 2, 3];
let mut iter = v.into_iter();
while let Some(x) = iter.next() {
    println!("{}", x);
}
```

<!-- reset_layout -->
<!-- end_slide -->

```rust
let v = vec![1, 2, 3, 4, 5];

let sum_of_squares: u64 = v.iter().map(|x| x * x).sum();
println!("sum_of_squares: {}", sum_of_squares);

let added_one: Vec<_> = v.iter().map(|x| x + 1).collect();
println!("added_one: {:?}", added_one);

for (i, x) in added_one.iter().filter(|&&x| x > 2 && x < 5).enumerate() {
    println!("({}, {})", i, x);
}

let product= v.iter().chain(added_one.iter()).fold(1, |acc, x| acc * x);
println!("product: {}", product);
```
```
sum_of_squares: 55
added_one: [2, 3, 4, 5, 6]
(0, 3)
(1, 4)
product: 86400
```
<!-- end_slide -->

Obsługa plików
===

```rust
let s = std::fs::read_to_string("in.txt").unwrap();
std::fs::write("out.txt", s).unwrap();
```
```rust
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    let in_f = File::open("in.txt").unwrap();
    let out_f = File::create("out.txt").unwrap();
    let in_buf = BufReader::new(in_f);
    let mut out_buf = BufWriter::new(out_f);
    for line in in_buf.lines() {
        let line = line.unwrap();
        println!("{}", line);
        writeln!(out_buf, "{}", line).unwrap();
    }
}
```

Patrz przykład `files_copy_contents.rs`

<!-- end_slide -->

Wątki
===

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust
use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("main thread: done");
    handle.join().unwrap();
    println!("main thread: joined");
}
```

<!-- column: 1 -->

```
main thread: 1
spawned thread: 1
main thread: 2
spawned thread: 2
main thread: 3
spawned thread: 3
main thread: 4
spawned thread: 4
main thread: done
spawned thread: 5
spawned thread: 6
spawned thread: 7
spawned thread: 8
spawned thread: 9
main thread: joined
```

<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [3, 1] -->

<!-- column: 0 -->

```rust
use std::{time::Duration, sync::{mpsc, Arc, Mutex}, thread};
fn main() {
    let m1 = Arc::new(Mutex::new(1));
    let m2 = m1.clone();
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move || {
        for _ in 0..10 {
            {
                let mut num = m1.lock().unwrap();
                *num += 1;
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(600));
            let num = m2.lock().unwrap();
            tx.send(*num).unwrap();
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    t1.join().unwrap();
    t2.join().unwrap();
}
```

<!-- column: 1 -->

```
Got: 3
Got: 4
Got: 5
Got: 6
Got: 7
Got: 9
Got: 10
Got: 11
Got: 11
Got: 11
```

<!-- reset_layout -->
<!-- end_slide -->

unsafe
===

```rust
use std::arch::asm;

unsafe fn dangerous_square(x: u32) -> u64 {
    let hi: u32;
    let lo: u32;
    asm!("mul eax", inlateout("eax") x => lo, out("edx") hi);
    ((hi as u64) << 32) | (lo as u64)
}

fn main() {
    let a = 1000000;
    let b = unsafe { dangerous_square(a) };
    println!("{a}^2 = {b}");

    let arr = [1, 2, 3, 4, 5];
    let p: *const i32 = &arr[0];
    let x = unsafe { *p.offset(3) };
    println!("x = {}", x);
}
```
```
1000000^2 = 1000000000000
x = 4
```
<!-- end_slide -->

async
===

Patrz przykład `horrible_async.rs`

<!-- end_slide -->

Dokumentacja i testy
===

```rust
/**
 * This function adds two numbers.
 * # Examples
 * 
 * ```
 * let x = 5;
 * let y = 3;
 * assert_eq!(8, add_numbers(x, y));
 * ```
 * ## this is markdown text
 * **bold** *italic*
 * - list1
 * - list2
 */
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add_numbers() {
    assert_eq!(8, add_numbers(5, 3));
}
```

<!-- end_slide -->

Makra
===

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
macro_rules! print_items {
    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn main() {
    let x = 5;
    print_items!(-2010, 12u8, 82, "a", x);
    print_items!("Hello", "World", "!");
}
```

<!-- column: 1 -->

```
-2010
12
82
a
5
Hello
World
!
```

<!-- reset_layout -->

Oprócz makr deklaratywnych Rust obsługuje także makra proceduralne,
które są pełnoprawnymi funkcjami transformującymi strumień tokenów,
wykonywanymi na etapie kompilacji.
<!-- end_slide -->

Zasoby edukacyjne
===

- "The Rust Programming Language" ([The Book](https://doc.rust-lang.org/stable/book/))
- "Rust by Example" (<https://doc.rust-lang.org/stable/rust-by-example/>)
- "Comprehensive Rust" (<https://google.github.io/comprehensive-rust/>)
- "The Rust Reference" (<https://doc.rust-lang.org/stable/reference/>)

Inne zasoby są także opisane na <https://www.rust-lang.org/learn>

<!-- end_slide -->

Przykłady praktyczne
===

<!-- end_slide -->

Koniec
===

# Koniec
Koniec
