use std::process;

//experiments

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

// tedge tests

pub fn expect_tedge_connect() {
    let connect = process::Command::new("sudo")
        .arg("tedge")
        .arg("connect")
        .arg("c8y")
        .status();
    let status = connect.unwrap();
    assert!(status.success());
}

pub fn expect_tedge_connect_test() {
    let connect = process::Command::new("sudo")
        .arg("tedge")
        .arg("connect")
        .arg("c8y")
        .arg("--test")
        .status();
    let status = connect.unwrap();
    assert!(status.success());
}

pub fn expect_tedge_disconnect() {
    let disconnect = process::Command::new("sudo")
        .arg("tedge")
        .arg("disconnect")
        .arg("c8y")
        .status();
    let status = disconnect.unwrap();
    assert!(status.success());
}

pub fn expect_mapper_enabled() {
    let mapper = process::Command::new("sudo")
        .arg("systemctl")
        .arg("status")
        .arg("tedge-mapper-c8y")
        .output();
    let status = mapper.unwrap().status.code().unwrap();
    assert_eq!(status, 0);
}

pub fn expect_mapper_disabled() {
    let mapper = process::Command::new("sudo")
        .arg("systemctl")
        .arg("status")
        .arg("tedge-mapper-c8y")
        .output();
    let status = mapper.unwrap().status.code().unwrap();
    assert_eq!(status, 3);
}

pub fn setup_connect_c8y() {
    expect_mapper_disabled();
    expect_tedge_connect();
    expect_mapper_enabled();
}

pub fn teardown_connect_c8y() {
    expect_tedge_disconnect();
    expect_mapper_disabled();
}

//////////////////////////////

pub struct TedgeConnectTest {}

impl TedgeConnectTest {
    pub fn new() -> TedgeConnectTest {
        println!("*** New TedgeConnectTest and calling setup_connect_c8y");
        setup_connect_c8y();
        TedgeConnectTest {}
    }
}

impl Drop for TedgeConnectTest {
    fn drop(&mut self) {
        teardown_connect_c8y();
        println!("*** Dropping TedgeConnectTest and called teardown_connect_c8y");
    }
}


//////////////////////

pub struct TestSetupMagicA {}

impl TestSetupMagicA {
    pub fn new() -> TestSetupMagicA {
        println!("*** New TestSetupMagicA");
        TestSetupMagicA{}
    }
}

impl Drop for TestSetupMagicA{
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicA");
    }
}


///

pub struct TestSetupMagicB {}

impl TestSetupMagicB {
    pub fn new() -> TestSetupMagicB {
        println!("*** New TestSetupMagicB");
        TestSetupMagicB{}
    }
}

impl Drop for TestSetupMagicB{
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicB");
    }
}

///

pub struct TestSetupMagicC {}

impl TestSetupMagicC {
    pub fn new() -> TestSetupMagicC {
        println!("*** New TestSetupMagicC");
        TestSetupMagicC{}
    }
}

impl Drop for TestSetupMagicC{
    fn drop(&mut self) {
        println!("*** Dropping TestSetupMagicC");
    }
}










