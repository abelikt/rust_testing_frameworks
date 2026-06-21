use injectorpp::interface::injector::*;
use std::any;
use std::ffi;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path;
use std::path::Path;
use std::process;
use std::time;

use std::os::unix::process::ExitStatusExt;

fn command_caller_stdout() -> (String, String) {
    let result = process::Command::new("ls")
        .arg("-l")
        .output()
        .expect("Command failed");
    (
        String::from_utf8(result.stdout).unwrap(),
        String::from_utf8(result.stderr).unwrap(),
    )
}

#[test]
fn test_command_caller_output() {
    let (stdout, stderr) = command_caller_stdout();
    assert!(stdout.contains("Cargo.toml"));
    assert!(stderr == "");
    {
        // arrange
        let mut injector = InjectorPP::new();
        injector
            .when_called(
                injectorpp::func!( fn(process::Command::output)( &mut process::Command )
                -> io::Result<process::Output>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn( _cmd: &mut process::Command )
                    -> io::Result<process::Output>,
                    returns: Ok(process::Output {
                        status: process::ExitStatus::from_raw(42),
                        stdout: Vec::from("Hello world!"),
                        stderr: Vec::from("Hello Errors!"),
                    }
                ),
                times: 1
            ));
        // act
        let (stdout, stderr) = command_caller_stdout();
        // assert
        println!("Stdout : {stdout}");
        assert_eq!(stdout, "Hello world!");
        assert_eq!(stderr, "Hello Errors!");
    }
}

fn patch_command_output() -> InjectorPP {
    let mut injector = InjectorPP::new();
    injector
            .when_called(
                injectorpp::func!( fn(process::Command::output)( &mut process::Command )
                -> io::Result<process::Output>),
            )
            .will_execute(injectorpp::fake!(
                func_type: fn( _cmd: &mut process::Command )
                    -> io::Result<process::Output>,
                    returns: Ok(process::Output {
                        status: process::ExitStatus::from_raw(42),
                        stdout: Vec::from("Hello world!"),
                        stderr: Vec::from("Hello Errors!"),
                    }
                ),
                times: 1
            ));
    injector
}

#[test]
fn test_command_caller_output_extracted() {
    let (stdout, stderr) = command_caller_stdout();
    assert!(stdout.contains("Cargo.toml"));
    assert!(stderr == "");
    {
        // arrange
        // Just assigning _ does not seem work.
        // The patch guard  is maybe not even stored in that case
        let _injector = patch_command_output();
        // act
        let (stdout, stderr) = command_caller_stdout();
        // assert
        println!("Stdout : {stdout}");
        assert_eq!(stdout, "Hello world!");
        assert_eq!(stderr, "Hello Errors!");
    }
}
