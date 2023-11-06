//! This crate contains the `surround!` macro.
//! 
//! # `surround!` usage
//! ```rust,ignore
//! surround!(num left right middle...)
//! ```
//! Where:
//! * `num` is a number literal with the number of times to repeat `left` and `right`
//! * `left` is the left string to repeat
//! * `right` is the right string to repeat
//! * `middle` is the middle token stream to surround
//! 
//! # Example
//! ```
//! use proc_macro_lib::surround;
//! assert_eq!(surround!(2 "Some(" ")" 4), Some(Some(4)));
//! assert_eq!(surround!(3 "Some(" ")" ["hello", "world"]), Some(Some(Some(["hello", "world"]))));
//! ```
extern crate proc_macro;

use std::str::FromStr;

use litrs::Literal;
use proc_macro::{TokenStream, TokenTree};

fn string_inside_literal(literal: TokenTree) -> String {
    if let Ok(Literal::String(s)) = Literal::try_from(literal) {
        s.into_value().to_string()
    } else {
        panic!("Expected string literal");
    }
}

#[proc_macro]
pub fn surround(stream: TokenStream) -> TokenStream {
    let mut iter = stream.into_iter();

    let num: usize = iter.next().unwrap().to_string().parse().unwrap();

    let left_string = string_inside_literal(iter.next().unwrap());
    let right_string = string_inside_literal(iter.next().unwrap());
    let middle_string = TokenStream::from_iter(iter).to_string();
    let mut result_string = String::new();
    result_string.reserve(left_string.len() + right_string.len() + middle_string.len());
    for _ in 0..num {
        result_string.push_str(&left_string);
    }
    result_string.push_str(&middle_string.to_string());
    for _ in 0..num {
        result_string.push_str(&right_string);
    }
    TokenStream::from_str(&result_string).unwrap()
}
