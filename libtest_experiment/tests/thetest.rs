/*
 *
 * https://crates.io/crates/easy_process
 * https://crates.io/crates/serial_test
 *
 */

#[cfg(test)]
mod int_tests {

    use libtest_experiment as t;

    #[test]
    // #[serial] // Disabled looks unmaintained
    #[should_panic]
    fn atest() {
        println!("Start of the test");
        let t = t::TestThing {};
        t.thing();
        let g = t::TestGadget {};
        g.gadget();

        panic!();
    }

    #[cfg(feature = "test-feature")]
    #[test]
    // #[serial]
    // #[ignore]
    fn connect() {
        // TODO add something nice
    }

    #[test]
    // #[serial]
    fn test_magic_abc() {
        // dependencies
        let _a = t::TestSetupMagicA::new();
        let _b = t::TestSetupMagicB::new();
        let _c = t::TestSetupMagicC::new();
        println!("The real test");
    }
}
