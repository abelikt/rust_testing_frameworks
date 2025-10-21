// Experiments

pub struct TestThing {}

impl TestThing {
    pub fn thing(&self) {
        println!("Thing");
    }
}

impl Drop for TestThing {
    fn drop(&mut self) {
        println!("Drop thing");
    }
}

pub struct TestGadget {}

impl TestGadget {
    pub fn gadget(&self) {
        println!("Gadget");
    }
}

impl Drop for TestGadget {
    fn drop(&mut self) {
        println!("Drop gadget");
    }
}

//////////////////////////////

pub struct EnvironmentTest {}

impl EnvironmentTest {
    pub fn new() -> EnvironmentTest {
        println!("*** New EnvironmentTest connect");
        // connect();
        EnvironmentTest {}
    }
}

impl Drop for EnvironmentTest {
    fn drop(&mut self) {
        // disconnect();
        println!("*** Dropping EnvironmentTest and called disconnect");
    }
}

//////////////////////

pub struct TestSetupMagicA {}

impl TestSetupMagicA {
    pub fn new() -> TestSetupMagicA {
        println!("*** New TestSetupMagicA");
        TestSetupMagicA {}
    }
}

impl Drop for TestSetupMagicA {
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicA");
    }
}

///

pub struct TestSetupMagicB {}

impl TestSetupMagicB {
    pub fn new() -> TestSetupMagicB {
        println!("*** New TestSetupMagicB");
        TestSetupMagicB {}
    }
}

impl Drop for TestSetupMagicB {
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicB");
    }
}

///

pub struct TestSetupMagicC {}

impl TestSetupMagicC {
    pub fn new() -> TestSetupMagicC {
        println!("*** New TestSetupMagicC");
        TestSetupMagicC {}
    }
}

impl Drop for TestSetupMagicC {
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicC");
    }
}
