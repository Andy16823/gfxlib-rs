pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod math;
use graphics::*;
pub mod graphics;
pub mod core;
pub mod shader;
pub mod utils;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
