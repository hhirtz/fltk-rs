 #![no_std]
 #![allow(non_camel_case_types)]
 #![allow(dead_code)]

pub mod fl;
pub mod widget;
pub mod group;
pub mod window;
pub mod button;
pub mod frame;
pub mod input;
pub mod output;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
