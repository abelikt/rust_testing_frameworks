// Half started experiment for injectorpp
//
// Blatantly copied from the docu of injectorpp
// https://docs.rs/injectorpp/0.4.0/injectorpp/

use injectorpp::interface::injector::*;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::process::Command;

fn try_repair() -> Result<(), String> {
    println!("Running try_repair ...");
    if let Err(e) = fs::create_dir_all("/tmp/target_files") {
        println!("Error while running try_repair (expected).");
        return Err(format!("Could not create directory: {:?}", e));
    }

    Ok(())
}

// Basic example from the docu
#[test]
fn basici_example() {
    assert!(try_repair().is_ok());

    let mut injector = InjectorPP::new();
    injector
        .when_called(
            injectorpp::func!(fn (fs::create_dir_all)(&'static str) -> std::io::Result<()>),
        )
        .will_execute(injectorpp::fake!(
            func_type: fn(path: &str) -> std::io::Result<()>,
            when: path == "/tmp/target_files",
            returns: Ok(()),
            times: 1
        ));

    assert!(try_repair().is_ok());
}

// Simple example with fs::exists
#[test]
fn basic_std_fs_exists_a() -> io::Result<()> {
    if let Ok(false) = fs::exists("nofile.txt") {
    } else {
        panic!("Assumption wrong, file is existing");
    }

    let mut injector = InjectorPP::new();
    injector
        .when_called(injectorpp::func!(fn (fs::exists)(&'static str) -> std::io::Result<bool>))
        .will_execute(injectorpp::fake!(
            func_type: fn(_path: &str) -> std::io::Result<bool>,
            returns: Ok(true),
            times: 1
        ));
    let result = fs::exists("nofile.txt");
    assert!(result.is_ok());
    assert!(result.unwrap() == true);
    Ok(())
}

fn file_exists(name: &str) -> bool {
    fs::exists(name).unwrap()
}

// Make sure we also can mock the dependency
#[test]
fn basic_std_fs_exists_b() -> io::Result<()> {
    let filename = "nofile.txg";
    assert!(!file_exists(filename));

    // Check if the mock gets cleaned up when the scope ends
    {
        let mut injector = InjectorPP::new();
        injector
            .when_called(injectorpp::func!(fn (fs::exists)(&'static str) -> std::io::Result<bool>))
            .will_execute(injectorpp::fake!(
                func_type: fn(_path: &str) -> std::io::Result<bool>,
                returns: Ok(true),
                times: 1
            ));
        assert!(file_exists(filename));
    }
    assert!(!file_exists(filename));

    Ok(())
}

fn command_caller_stdout() -> String {
    let result = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Command failed");
    String::from_utf8(result.stdout).unwrap()
}

#[test]
fn command_caller_1() {
    let stdout = command_caller_stdout();
    stdout.contains("Cargo.toml");
}

#[test]
fn func_macro_calls() {
    injectorpp::func!(fn (fs::exists)(&'static str) -> std::io::Result<bool>);

    // Signature of output
    // pub fn output(&mut self) -> Result<Output>
    injectorpp::func!( fn(std::process::Command::output)( &mut std::process::Command ) -> io::Result<std::process::Output>);

    // Signature of the new function
    // pub fn new<S: AsRef<OsStr>>(program: S) -> Command
    // injectorpp::func!( fn(std::process::Command::new::<&std::ffi::OsString>)( &std::ffi::OsString ) -> Command)
}

#[test]
fn command_caller_2() {
    let stdout = command_caller_stdout();
    println!("{}", std::any::type_name::<Option<String>>());
    println!("{}", std::any::type_name::<std::process::Command>());
    println!(
        "{}",
        std::any::type_name_of_val(&std::process::Command::new::<&std::ffi::OsString>)
    );
    println!(
        "{}",
        std::any::type_name_of_val(&std::process::Command::new::<&std::ffi::OsString>)
    );

    let f = std::process::Command::new::<&std::ffi::OsString>;
    println!("{}", std::any::type_name_of_val(&f));
    type _Ff = fn(&OsString) -> std::process::Command;
    stdout.contains("Cargo.toml");
    {
        // let mut injector = InjectorPP::new();
        // injector.when_called(
        // could work but does not
        // injectorpp::func!( fn(std::process::Command::new::<&std::ffi::OsString>)( &std::ffi::OsString ) -> Command),

        // injectorpp::func!( fn(std::process::Command::new::<&std::ffi::OsString>)( &std::ffi::OsString ) -> Command),
        // );
        // .will_execute(injectorpp::fake!(
        // func_type: fn(_path: &str) -> std::io::Result<bool>,
        // returns: Ok(true),
        // times: 1
        // ));
    }
}
