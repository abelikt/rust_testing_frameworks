use std::process;

#[cfg(test)]
mod cmdargs {

    use std::any::type_name_of_val;

    use super::*;

    #[test]
    fn test_echo_pipes() {
        let call = process::Command::new("echo")
            .arg("hello_world")
            .output()
            .expect("Should have worked");

        println!("Type name of output {}", type_name_of_val(&call.stdout));
        let outp = String::from_utf8(call.stdout).unwrap();

        println!("Status {:?}", call.status);
        println!("Stdout {:?}", outp);
        println!("Stderr {:?}", call.stderr);
        assert!(call.status.success());

        let output = outp.trim();
        assert_eq!(output, String::from("hello_world"));
    }
}
