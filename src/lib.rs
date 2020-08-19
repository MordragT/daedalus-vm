#![feature(vec_remove_item)]

pub mod game_state;
pub mod stdlib;
pub mod vm;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
