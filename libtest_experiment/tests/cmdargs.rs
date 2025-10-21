use std::process;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_echo_pipes() {
        let call = process::Command::new("echo")
            .arg("hello_world")
            .output()
            .expect("Should have worked");

        let outp = String::from_utf8(call.stdout).unwrap();
        println!("Status {:?}", call.status);
        println!("Stdout {:?}", outp);
        println!("Stderr {:?}", call.stderr);
        assert!(call.status.success());
        assert_eq!(outp.trim(), String::from("hello_world"));
    }
}
