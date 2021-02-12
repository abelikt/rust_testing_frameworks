
// https://docs.rs/mockall/0.6.0/mockall/
// It can mock most traits, or structs that only have a single impl block. For things it can't handle, there is mock!.

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

use mockall::*;
use mockall::predicate::*;

//

// cargo test -- --show-output


// https://docs.rs/mockall/0.6.0/mockall/macro.mock.html

// https://docs.rs/crate/mockall_examples/0.7.2/source/src/lib.rs

// more
// https://github.com/kriomant/mockers
// https://www.lpalmieri.com/posts/2020-04-13-wiremock-async-http-mocking-for-rust-applications/

// https://users.rust-lang.org/t/mocking-library/5295
// https://github.com/kriomant/mockers

// https://github.com/asomers/mockall

// mockall

// https://stackoverflow.com/questions/55152927/how-to-mock-specific-methods-but-not-all-of-them-in-rust
// As you have already learned, you cannot replace methods on a type.
// The only thing you can do is move the methods to a trait and then
// provide production and test-specific implementations of that trait.
// How you structure the trait determines the granularity of what you are able to test.

/*********************************************************************/


// https://knowitlabs.no/rust-2020-testing-4ab3d80112ba

/*
 * Within my team it has been debated whether the best style is to
 exclusively depend on abstract interfaces, everywhere. My personal
 view is that this should be determined by the runtime intent. In high
 level languages this is usually no problem, because method dispatch is
 usually dynamic dy default. This means that any or most concrete types
 or classes are easily sub-classable, and thus the coupling to such a
 class by name is not really that strong under the hood. All
 python/Java/Kotlin/JS languages support this easily: Creating mocks of
 concrete types.
 */

/*
Higher order functions

If I want to test a function or procedure A in isolation that depends
on another function B, I can pass B as a parameter to A. In production
code I call A(B). In the test I call A(mocked_B), now with full
control over that dependency.

*/

/*
...
So the way to express an abstraction in Rust is to use a trait.
* Trait mocking in Rust today is fairly developer friendly, but not as
* friendly as in dynamic-dispatch-languages. What I’ve been doing so
* far is to introduce a new trait everywhere I need test isolation,
* and use the crate mockall to autogenerate a mocked implementation
* that I instantiate in my test. Not very far from how java/mockito or
* jest works, except that, as mentioned, we always need to be very
* explicit about the abstraction taking place:

*/

/*
* In Rust, making code dynamic dispatch is less verbose than making
* it static dispatch. This is in some ways counterintuitive, because
* in other areas of the language it appears to be a concious decision
* that less verbose code is simpler and therefore more efficient. Heap
* allocation is one example, it is always very apparent that a heap
* allocation will take place: Box<MyThing> instead of MyThing. A heap
* allocation is more expensive to type and execute. It seems concious
* that there is no short hand for this. First consider dynamic dispatch:
*/


/*********************************************************************/

// Inject the depenency as a trait
// call via associated function

#[automock]
trait Readable2 {
    fn read_hw2(&self) -> i32;
}

struct Sensor2 {}

impl Readable2 for Sensor2 {
    fn read_hw2(&self) -> i32 {
        rand::thread_rng().gen_range(1,11)
    }
}

struct Speedometer3 {
    velocity3:i32,
    // sensor3 the external dependency we like to mock
    sensor3: Box<dyn Readable3 >, // Trait object: Any sensor that implements Readable 3
    factor3:i32,
}

impl Speedometer3 {

    fn get_velocity3(&self) -> i32 {
        self.sensor3.read_hw3() * self.factor3
    }

    fn new (val:i32) -> Speedometer3{
        Speedometer3 {
            velocity3:0,
            sensor3: Box::new( Sensor3{} ),
            factor3: val
        }
    }
}

fn test_speedo3 ()
{
    // this is the mocked sensor
    let mut mock_readable3 = MockReadable3::new();

    mock_readable3.expect_read_hw3()
        .with()
        .times(2)
        .returning( || 43 );

    let speedo3 =Speedometer3{
        velocity3:0,
        sensor3: Box::new( mock_readable3 ),
        factor3:5 };

    println!("Velocity3 is {}", speedo3.get_velocity3( ));
    assert_eq!(speedo3.get_velocity3(), 215);
}

fn test_speedo3_normal_use_case_a ()
{
    let mysensor = Sensor3{};
    let speedo3 = Speedometer3 {
        velocity3:0,
        sensor3: Box::new( mysensor ),
        factor3:2
    };

    println!("Velocity use case b is {}", speedo3.get_velocity3( ));
}

fn test_speedo3_normal_use_case_b ()
{

    let speedo3 = Speedometer3 {
        velocity3:0,
        sensor3: Box::new( Sensor3{} ),
        factor3:2
    };

    println!("Velocity use case b is {}", speedo3.get_velocity3( ));
}

fn test_speedo3_normal_use_case_c ()
{

    let speedo3 = Speedometer3::new( 1 );

    println!("Velocity use case b is {}", speedo3.get_velocity3( ));
}


#[cfg(test)]
mod test_mod_speedo3 {

    use super::*;

    #[test]
    fn def_testspeedo3_a() {

        // Arrange
        let mut mock_readable3 = MockReadable3::new();

        mock_readable3.expect_read_hw3()
            .with()
            .times(1)
            .returning( || 45 );

        let speedo3 = Speedometer3{
            velocity3:0,
            sensor3: Box::new( mock_readable3 ),
            factor3:5 };

        // Act
        assert_eq!(speedo3.get_velocity3(), 225);

        // Assert
    }

    #[test]
    fn def_testspeedo3_annotated() {

        // Arrange

        // Create the mock
        let mut mock_readable3 = MockReadable3::new();

        // Configure the mock
        mock_readable3.expect_read_hw3()
            .with()
            .times(1)
            .returning( || 45 );

        // Crate our DUT
        let speedo3 = Speedometer3{
            velocity3:0,
            sensor3: Box::new( mock_readable3 ),
            factor3:5 };

        // Act: Call DUT

        let result = speedo3.get_velocity3();

        // Assert : Check further assertions

        assert_eq!( result, 225);
    }

}

/*********************************************************************/

fn main() {

    test_speedo3();
    test_speedo3_normal_use_case_a();
    test_speedo3_normal_use_case_b();
    test_speedo3_normal_use_case_c();

}
