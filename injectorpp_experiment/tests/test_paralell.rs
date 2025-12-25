// https://docs.rs/injectorpp/0.4.0/injectorpp/

use injectorpp::interface::injector::*;

fn simple_dut_dependency(input: u32) -> u32 {
    input * 2
}

fn dut_simple(input: u32) -> u32 {
    simple_dut_dependency(input) * 3
}

fn gen_injector() -> InjectorPP {
    let mut injector = InjectorPP::new();
    injector
        .when_called(injectorpp::func!(fn (simple_dut_dependency)(u32) -> u32 ))
        .will_execute(injectorpp::fake!(
            func_type: fn(input:u32) -> u32,
            returns: 8,
            times: 1
        ));
    injector
}

#[ignore]
#[test]
fn test_simple_dependency() {
    assert_eq!(dut_simple(2), 12);
    let _injector = gen_injector();
    assert_eq!(dut_simple(2), 24);
}

#[ignore]
#[test]
fn test_simple_dependency_2() {
    assert_eq!(dut_simple(2), 12);
    let _injector = gen_injector();
    assert_eq!(dut_simple(2), 24);
}

#[test]
fn test_simple_dependency_all() {
    {
        assert_eq!(dut_simple(2), 12);
        let _injector = gen_injector();
        assert_eq!(dut_simple(2), 24);
    }
    {
        assert_eq!(dut_simple(2), 12);
        let _injector = gen_injector();
        assert_eq!(dut_simple(2), 24);
    }
}
