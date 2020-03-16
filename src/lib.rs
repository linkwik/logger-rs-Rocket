#[macro_use] extern crate log;

#[doc(hidden)] #[macro_use] pub mod logger;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
