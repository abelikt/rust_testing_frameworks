#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
// Half started experiment for injectorpp
//
// Blatantly copied from the docu of injectorpp
// https://docs.rs/injectorpp/0.4.0/injectorpp/

// Observe renerated code with:
//
// cargo rustc --profile=check -- -Zunpretty=expanded > expanded/expand_main.rs
//
// Actually most of the content here exists so that we can observe the expanded macros.
// don't expect too much.

use std::fs;
use std::io;
use std::path;

use injectorpp::interface::injector::*;

fn try_repair() -> Result<(), String> {











    // injectorpp::func!(fs::create_dir_all, fn(&'a path::Path) -> io::Result<()>);
    //
    // Expanded macro from above code
    // Error: requires that `'a` must outlive `'static`
    //

    { ::std::io::_print(format_args!("Running try_repair ...\n")); };
    if let Err(e) = fs::create_dir_all("/tmp/target_files") {
        {
            ::std::io::_print(format_args!("Error while running try_repair (expected).\n"));
        };
        return Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Could not create directory: {0:?}",
                                e))
                    }));
    }
    Ok(())
}
fn example_a() {
    if !try_repair().is_ok() {
        ::core::panicking::panic("assertion failed: try_repair().is_ok()")
    };
    let mut injector = InjectorPP::new();
    injector.when_called({
                {
                    {
                        let fn_val: fn(&'static str) -> std::io::Result<()> =
                            fs::create_dir_all;
                        let ptr = fn_val as *const ();
                        let sig = std::any::type_name_of_val(&fn_val);
                        let type_id =
                            std::any::TypeId::of::<fn(&'static str)
                                        -> std::io::Result<()>>();
                        unsafe { FuncPtr::new_with_type_id(ptr, sig, type_id) }
                    }
                }
            }).will_execute({
            use std::sync::atomic::{AtomicUsize, Ordering};
            static FAKE_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let verifier =
                CallCountVerifier::WithCount {
                    counter: &FAKE_COUNTER,
                    expected: 1,
                };
            fn fake(path: &'static str) -> std::io::Result<()> {
                if path == "/tmp/target_files" {
                    let prev = FAKE_COUNTER.fetch_add(1, Ordering::SeqCst);
                    if prev >= 1 {
                        {
                            ::core::panicking::panic_fmt(format_args!("Fake function defined at {0}:{1}:{2} called more times than expected",
                                    "injectorpp_experiment/src/main.rs", 37u32, 23u32));
                        };
                    }
                    Ok(())
                } else {
                    {
                        ::core::panicking::panic_fmt(format_args!("Fake function defined at {0}:{1}:{2} called with unexpected arguments",
                                "injectorpp_experiment/src/main.rs", 37u32, 23u32));
                    };
                }
            }
            let f: fn(&'static str) -> std::io::Result<()> = fake;
            let raw_ptr = f as *const ();
            (unsafe { FuncPtr::new(raw_ptr, std::any::type_name_of_val(&f)) },
                verifier)
        });
    if !try_repair().is_ok() {
        ::core::panicking::panic("assertion failed: try_repair().is_ok()")
    };
}
fn example_b() {
    if !try_repair().is_ok() {
        ::core::panicking::panic("assertion failed: try_repair().is_ok()")
    };
    let mut injector = InjectorPP::new();
    injector.when_called({
                {
                    {
                        let fn_val: fn(&'static str) -> std::io::Result<()> =
                            fs::create_dir_all;
                        let ptr = fn_val as *const ();
                        let sig = std::any::type_name_of_val(&fn_val);
                        let type_id =
                            std::any::TypeId::of::<fn(&'static str)
                                        -> std::io::Result<()>>();
                        unsafe { FuncPtr::new_with_type_id(ptr, sig, type_id) }
                    }
                }
            }).will_execute({
            use std::sync::atomic::{AtomicUsize, Ordering};
            static FAKE_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let verifier =
                CallCountVerifier::WithCount {
                    counter: &FAKE_COUNTER,
                    expected: 2,
                };
            fn fake(path: &'static str) -> std::io::Result<()> {
                if path == "/tmp/target_files" {
                    let prev = FAKE_COUNTER.fetch_add(1, Ordering::SeqCst);
                    if prev >= 2 {
                        {
                            ::core::panicking::panic_fmt(format_args!("Fake function defined at {0}:{1}:{2} called more times than expected",
                                    "injectorpp_experiment/src/main.rs", 55u32, 23u32));
                        };
                    }
                    Err(std::io::Error::new(std::io::ErrorKind::Deadlock,
                            "Oh no, a mock"))
                } else {
                    {
                        ::core::panicking::panic_fmt(format_args!("Fake function defined at {0}:{1}:{2} called with unexpected arguments",
                                "injectorpp_experiment/src/main.rs", 55u32, 23u32));
                    };
                }
            }
            let f: fn(&'static str) -> std::io::Result<()> = fake;
            let raw_ptr = f as *const ();
            (unsafe { FuncPtr::new(raw_ptr, std::any::type_name_of_val(&f)) },
                verifier)
        });
    if !try_repair().is_err() {
        ::core::panicking::panic("assertion failed: try_repair().is_err()")
    };
    { ::std::io::_print(format_args!("Error type: {0:?}\n", try_repair())); };
}
fn simple_dut_dependency(input: u32) -> u32 { input * 2 }
fn dut_simple(input: u32) -> u32 { simple_dut_dependency(input) * 3 }
fn example_c() {
    let mut injector = InjectorPP::new();
    injector.when_called({
                {
                    {
                        let fn_val: fn(u32) -> u32 = simple_dut_dependency;
                        let ptr = fn_val as *const ();
                        let sig = std::any::type_name_of_val(&fn_val);
                        let type_id = std::any::TypeId::of::<fn(u32) -> u32>();
                        unsafe { FuncPtr::new_with_type_id(ptr, sig, type_id) }
                    }
                }
            }).will_execute({
            use std::sync::atomic::{AtomicUsize, Ordering};
            static FAKE_COUNTER: AtomicUsize = AtomicUsize::new(0);
            let verifier =
                CallCountVerifier::WithCount {
                    counter: &FAKE_COUNTER,
                    expected: 1,
                };
            fn fake(input: u32) -> u32 {
                if true {
                    let prev = FAKE_COUNTER.fetch_add(1, Ordering::SeqCst);
                    if prev >= 1 {
                        {
                            ::core::panicking::panic_fmt(format_args!("Fake function defined at {0}:{1}:{2} called more times than expected",
                                    "injectorpp_experiment/src/main.rs", 77u32, 23u32));
                        };
                    }
                    88
                } else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
            }
            let f: fn(u32) -> u32 = fake;
            let raw_ptr = f as *const ();
            (unsafe { FuncPtr::new(raw_ptr, std::any::type_name_of_val(&f)) },
                verifier)
        });
    match (&dut_simple(11), &264) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val,
                    &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn example_d<'a>() {
    let fn_val: fn(&'a path::Path) -> io::Result<()> = fs::create_dir_all;
    let ptr = fn_val as *const ();
    let sig = std::any::type_name_of_val(&fn_val);
    let type_id =
        std::any::TypeId::of::<fn(&'a path::Path) -> io::Result<()>>();
    let _funptr = unsafe { FuncPtr::new_with_type_id(ptr, sig, type_id) };
}
fn main() {
    { ::std::io::_print(format_args!("Hello, Example!\n")); };
    example_a();
    example_b();
    example_c();
    example_d();
}
