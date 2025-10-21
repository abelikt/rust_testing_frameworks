/*
Test to play with the drop of the magic structs

    cargo test test_panic -- --nocapture

    cargo test test_panic -- --nocapture --test-threads 1

*/

#[cfg(test)]
mod panic_tests {

    //use super::*;
    use libtest_experiment as t;

    #[test]
    fn test_no_panic() {
        println!("One");
        println!("Two");
        println!("Tree");
        println!("Four");
    }

    #[should_panic]
    #[test]
    fn test_panic() {
        println!("One");
        println!("Two");
        panic!();
    }

    #[test]
    fn test_panic_magic() {
        println!("One");
        let _ta = t::TestSetupMagicA::new();
        println!("Two");
        println!("Tree");
        println!("Four");
    }

    #[should_panic]
    #[test]
    fn test_panic_magic_with_panic() {
        println!("One");
        let _ta = t::TestSetupMagicA::new();
        println!("Two");
        panic!();
    }
}
