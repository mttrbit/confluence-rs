#[macro_use]
mod macros;

#[macro_use]
extern crate error_chain;

pub mod client;
pub mod errors;
pub mod model;

mod content;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
