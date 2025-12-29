//! [![github]](https://github.com/Tom-game-project/error-combinator)&ensp;[![crates-io]](https://crates.io/crates/error-combinator)&ensp;[![docs-rs]](https://docs.rs/error-combinator/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! # Example
//!
//! ```
//! use error_combinator::{
//!     check::{
//!         Check,        // trait
//!         CheckOutcome, // struct
//!         CheckState    // struct
//!     },
//! };
//! 
//! use std::marker::PhantomData;
//! 
//! struct Checked;
//! struct Unchecked;
//! struct ErrState<CheckStartsWithHello> {
//!     _check_starts_with_hello: PhantomData<CheckStartsWithHello>,
//! }
//! 
//! #[derive(Debug)]
//! enum ValidateErr {
//!     CheckStartsWithHelloErr,
//! }
//! 
//! /// function that has `Fn(CheckState<T, Pre>) -> CheckOutcome<T, State, E>` type 
//! /// be implemented `Check trait` automatically
//! /// You can call following method.
//! /// ```
//! /// let r = checker.check(
//! ///     CheckState::new(s)
//! /// );
//! /// ```
//! fn check_starts_with_hello(
//!     data: CheckState<&str, ErrState<Unchecked>>) 
//! -> 
//! CheckOutcome<&str, ErrState<Checked>, ValidateErr>
//! {
//!     if data.value.starts_with("hello") {
//!         CheckOutcome::Passed(
//!             CheckState::new(data.value)
//!         )
//!     } else {
//!         CheckOutcome::Failed{
//!             state: CheckState::new(data.value),
//!             err: ValidateErr::CheckStartsWithHelloErr
//!         }
//!     }
//! }
//! 
//! fn main() {
//!     let s= "hello abc world";
//!     let checker = check_starts_with_hello; 
//! 
//!     // call `Check` trait method
//!     let r = checker.check(
//!         CheckState::new(s)
//!     );
//!     println!("test case: \"{}\"", s);
//!     match r {
//!         CheckOutcome::Passed(_v) => {
//!             println!("Passed!");
//!         }
//!         CheckOutcome::Failed{state:_, err} => {
//!             println!("Failed because");
//!             println!("{:?}", err)
//!         }
//!     }
//!     println!("---")
//! }
//! 
//! ```

pub mod check;
pub mod cmberr;

