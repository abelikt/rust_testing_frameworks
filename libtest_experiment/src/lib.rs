use std::process;

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

// Environment tests

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

// Set-Up Tests A

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

// Set-Up Tests B

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

// Set-Up Tests C

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

// Import experiments to mock stuff
// from: https://stackoverflow.com/questions/77855780/how-to-mock-stdsomething-in-rust#77857356

#[allow(unused_imports)]
use std::path::Path;

#[cfg(not(test))]
use std::fs::read;

#[allow(unused_variables)]
#[cfg(test)]
fn read(p: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
    Ok("hello".as_bytes().to_owned())
}

#[allow(dead_code)]
fn example() -> Vec<u8> {
    let data = read("path").unwrap();
    data
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_read() {
        assert_eq!(super::example(), "hello".as_bytes().to_owned());
    }
}

// Furhter mocking experiments

pub fn call_ls(arg: &str) -> String {
    let call = process::Command::new("ls")
        .arg(arg)
        .output()
        .expect("Should have worked");

    let outp = String::from_utf8(call.stdout).unwrap();
    println!("{}", outp);
    outp
}
