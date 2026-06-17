#[cfg(test)]
mod cmdargs {

    use libtest_experiment as t;
    use std::any;
    use std::process;

    #[test]
    fn test_echo_pipes() {
        let call = process::Command::new("echo")
            .arg("hello_world")
            .output()
            .expect("Should have worked");

        println!(
            "Type name of output {}",
            any::type_name_of_val(&call.stdout)
        );
        let outp = String::from_utf8(call.stdout).unwrap();

        println!("Status {:?}", call.status);
        println!("Stdout {:?}", outp);
        println!("Stderr {:?}", call.stderr);
        assert!(call.status.success());

        let output = outp.trim();
        assert_eq!(output, String::from("hello_world"));
    }

    #[test]
    fn test_cmd_stderr() {
        let call = process::Command::new("ls")
            .arg("nowhere")
            .output()
            .expect("Should have worked");

        println!(
            "Type name of output {}",
            any::type_name_of_val(&call.stdout)
        );
        let outp = String::from_utf8(call.stdout).unwrap();
        let err = String::from_utf8(call.stderr).unwrap();

        println!("Status {:?}", call.status);
        println!("Stdout {:?}", outp);
        println!("Stderr {:?}", err);
        assert!(!call.status.success());

        let output = outp.trim();
        let outpute = err.trim();
        assert_eq!(output, String::from(""));
        assert!(outpute.contains("No such file or directory"));
    }

    #[test]
    fn test_call_ls() {
        assert!(t::call_ls(".").contains("Cargo.toml"));
    }
}
