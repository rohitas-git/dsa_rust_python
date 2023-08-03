/*
    Rust tests will fail in two situations:
    - Panicking (which is where something has gone wrong and can be fixed), or
    - Returning an Err result
*/

mod panicking {

    // The standard library provides macros that will panic under the right conditions:

    // check if something is true
    assert!(boolean_expression, "message when false");

    // check if one thing is equal to another
    assert_eq!(expected, actual, "message when not equal");

    // check if two things are not equal
    assert_ne!(expr1, expr2, "message when equal");

    // unconditional panic
    panic!("message");

    // example
    #[test]
    fn this_test_fails() {
        assert_eq!(1, 2);
    }
}

/*
    Since a test needs to panic to fail,
    the .expect() on the Result and Option types are great for testing:

    If you don't want to .unwrap() or .expect() on Result,
    you can instead change the return type of a test function to Result<T, E>
    which will trigger a test failure whenever Err gets returned.
*/

mod returning_error {
    fn some_fn() -> Result<bool, String> {
        Ok(true)
    }

    #[test]
    fn result_test() -> Result<(), String> {
        // We can use question mark instead of unwrap.
        // If some_fn() is `Err`, then the test will
        // fail at this line.
        let is_ok = some_fn()?;

        if is_ok {
            Ok(())
        } else {
            // `Err` fails the test
            Err("not ok!".into())
        }
    }
}

mod special_situation{

    mod should_panic{
        // Sometimes you may want to test that some code does panic. 
        // In these cases you can add the #[should_panic] attribute, 
        // which fails the test whenever the test code does not panic:
        #[test]
        #[should_panic]
        fn panic_ok() {
            panic!("test passed");
        }
        
        #[test]
        #[should_panic]
        fn this_fails() {
          assert!(true);
        }
    }

    mod skip_test{
        // There are also times when running a test takes a significant amount of time to execute. 
        // For these situations, the #[ignore] attribute 
        // will cause the test to get skipped when running cargo test:
        #[test]
        #[ignore]
        fn only_runs_with_flags() {
            std::thread::sleep(std::time::Duration::from_secs(5000));
            panic!("test failed");
        }

        // To then come back and run ignored tests, 
        // invoke cargo with the --ignored flag and then go grab a coffee ☕.
        // >> $ cargo test -- --ignored
    }
}

mod fluent_testing{
    // While not part of the standard library, the spectral crate provides a fluent testing API for Rust:
    // $ cargo add spectral

    #[test]
    fn with_spectral() {
        use spectral::prelude::*;
        assert_that(&1).is_equal_to(2);

        let nums = vec![1, 2, 3];
        assert_that(&nums).has_length(3);
        assert_that(&nums).contains(1);
    }
}