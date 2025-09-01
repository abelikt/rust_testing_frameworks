// Blatantly copied from the docu of injectorpp
// https://docs.rs/injectorpp/0.4.0/injectorpp/

use std::fs;

use injectorpp::interface::injector::*;

fn try_repair() -> Result<(), String> {
    println!("Running try_repair ...");
    if let Err(e) = fs::create_dir_all("/tmp/target_files") {
        println!("Error while running try_repair (expected).");
        return Err(format!("Could not create directory: {}", e));
    }

    Ok(())
}

fn main() {
    println!("Hello, Example!");

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
