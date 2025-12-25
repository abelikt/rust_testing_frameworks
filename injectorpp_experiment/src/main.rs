// Half started experiment for injectorpp
//
// Blatantly copied from the docu of injectorpp
// https://docs.rs/injectorpp/0.4.0/injectorpp/

// Observe renerated code with:
//
// cargo rustc --profile=check -- -Zunpretty=expanded > expand.rs

use std::fs;

use injectorpp::interface::injector::*;

fn try_repair() -> Result<(), String> {
    println!("Running try_repair ...");
    if let Err(e) = fs::create_dir_all("/tmp/target_files") {
        println!("Error while running try_repair (expected).");
        return Err(format!("Could not create directory: {:?}", e));
    }

    Ok(())
}

fn example_a() {
    assert!(try_repair().is_ok());

    let mut injector = InjectorPP::new();
    injector
        .when_called(
            injectorpp::func!(fn (fs::create_dir_all)(&'static str) -> std::io::Result<()>),
        )
        .will_execute(injectorpp::fake!(
            func_type: fn(path: &'static str) -> std::io::Result<()>,
            when: path == "/tmp/target_files",
            returns: Ok(()),
            times: 1
        ));

    assert!(try_repair().is_ok());
}

fn example_b() {
    assert!(try_repair().is_ok());

    let mut injector = InjectorPP::new();
    injector
        .when_called(
            injectorpp::func!(fn (fs::create_dir_all)(&'static str) -> std::io::Result<()>),
        )
        .will_execute(injectorpp::fake!(
            func_type: fn(path: &'static str) -> std::io::Result<()>,
            when: path == "/tmp/target_files",
            returns: Err(std::io::Error::new(std::io::ErrorKind::Deadlock, "Oh no, a mock")),
            times: 2
        ));

    assert!(try_repair().is_err());
    println!("Error type: {:?}", try_repair());
}
fn simple_dut_dependency(input: u32) -> u32 {
    input * 2
}

fn dut_simple(input: u32) -> u32 {
    simple_dut_dependency(input) * 3
}

fn example_c() {
    let mut injector = InjectorPP::new();
    injector
        .when_called(injectorpp::func!(fn (simple_dut_dependency)(u32) -> u32 ))
        .will_execute(injectorpp::fake!(
            func_type: fn(input:u32) -> u32,
            returns: 88,
            times: 1
        ));

    assert_eq!(dut_simple(11), 264);
}

fn main() {
    println!("Hello, Example!");
    example_a();
    example_b();
    example_c();
}
