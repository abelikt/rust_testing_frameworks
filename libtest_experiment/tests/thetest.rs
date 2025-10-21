/*
 *
 * https://crates.io/crates/easy_process
 *
 *
 * Covers most of the things in :
 * home/micha/thin-edge.io/tests/PySys/environments/environment_tedge.py
 *
 * cargo test --features tedge-test -- --test-threads 1 --nocapture
 *
 */

#[cfg(test)]
mod int_tests {

    // use serial_test::serial;
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

    #[cfg(feature = "tedge-test")]
    #[test]
    // #[serial]
    // #[ignore]
    fn c8y_connect() {
        t::setup_connect_c8y();
        t::teardown_connect_c8y();
    }

    #[cfg(feature = "tedge-test")]
    #[test]
    // #[serial]
    // #[ignore]
    fn c8y_connect_magic() {
        let _tedge = t::TedgeConnectTest::new();
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
