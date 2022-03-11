use std::process;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tedge_v_verbose() {
        let call = process::Command::new("tedge")
            .arg("-V")
            .output()
            .expect("Should have worked");

        let outp = String::from_utf8(call.stdout).unwrap();
        println!("Status {:?}", call.status);
        println!("Stdout {:?}", outp);
        println!("Stderr {:?}", call.stderr);
        assert!(call.status.success());
        assert_eq!(outp.trim(), String::from("tedge 0.5.4"));
    }

    #[test]
    fn test_tedge_v() {
        let call = process::Command::new("tedge")
            .arg("-V")
            .output()
            .expect("Should have worked");
        let outp = String::from_utf8(call.stdout).unwrap();
        assert!(call.status.success());
        assert_eq!(outp.trim(), String::from("tedge 0.5.4"));
    }
}
