//use rstest::*;

pub mod powerfixture {

    use rstest::*;

    #[fixture]
    #[once]
    pub fn super_fixture() -> i32 {
        println!("Only once");
        42
    }
}
