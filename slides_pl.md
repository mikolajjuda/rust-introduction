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
- bezpieczeństwo pamięci bez potrzeby automatycznego odśmiecania (borrow checker)
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
fn main() { // main function is the entry point
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

## `str`
- sekwencja bajtów w kodowaniu UTF-8 o nieznanym rozmiarze
- zwykle używany poprzez referencję `&str` (wycinek napisu)

# Typ jednostkowy `()`
- pusta krotka
- zerowy rozmiar
- jedna możliwa wartość

<!-- reset_layout -->
<!-- end_slide -->

Zmienne
---

```rust
fn takes_i64(_x: i64) {}

fn main() {
    let 𐐘: i32; // explicit type i32 (i32 is also the default for integers)
    let y = 10u8; // type inferred from literal u8
    let mut a = 1; // variables are immutable by default
    let b = 2;
    let _草泥马: (); // underscore prefix suppresses unused variable warning
    let _żółć: ();

    a += y + 1; // type of a inferred here u8
    takes_i64(b); // type of b inferred here i64
    𐐘 = 5; // variables can be initialised after declaration

    println!("𐐘 = {}", 𐐘);
    println!("y = {}", y);
    println!("a = {}", a);
    println!("b = {}", b);
}
```

<!-- end_slide -->

Przepływ sterowania
---
