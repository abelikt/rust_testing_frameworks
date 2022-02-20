extern crate proc_macro;

mod things;
use crate::things::modthings;


fn main() {
    println!("Hello, world!");
    modthings::things();
}


#[cfg(test)]
mod tests {

    #[test]
    fn atest() {}

    #[test]
    fn atest1() {}
    #[test]
    fn atest2() {}
    #[test]
    fn atest3() {}
    #[test]
    fn atest4() {}
    #[test]
    fn atest5() {}
    #[test]
    fn atest6() {}
    #[test]
    fn atest7() {}
}
