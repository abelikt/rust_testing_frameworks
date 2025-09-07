// Half started experiment for injectorpp
//
// Blatantly copied from the docu of injectorpp
// https://docs.rs/injectorpp/0.4.0/injectorpp/

use std::fs;
use std::io;

use injectorpp::interface::injector::*;

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
            .when_called(
                injectorpp::func!(fn (fs::exists)(&'static str) -> std::io::Result<bool>),
            )
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
    assert!(!file_exists("nofile.txt"));

    {
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!(fn (fs::exists)(&'static str) -> std::io::Result<bool>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn(_path: &str) -> std::io::Result<bool>,
                returns: Ok(true),
                times: 1
            ));
        assert!(file_exists("nofile.txt"));
    }
    assert!(!file_exists("nofile.txt"));

    Ok(())
}
