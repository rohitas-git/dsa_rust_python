# How to Write Test

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

Set up any needed data or state.
Run the code you want to test.
Assert the results are what you expect.

## Failing tests
Rust tests will fail in two situations:
- Panicking (which is where something has gone wrong and can be fixed), or
- Returning an Err result

# [cfg(test)]
By stating #[cfg(test)], we restrict Rust to compiling the following code if you run the project with the command cargo test.

View "test_demo.rs" for this section


## Type of Testing
- Unit Testing
- Integration Testing
- Snapshot testing
- Mock Testing
- Property testing
- Fuzz Testing

# Test Naming 

## Best Practices for Test Case Names
- Use human-readable titles. 
- Be unique. (Try to separate similar test cases by including its variant description or intent.)

A common naming convention is to use the format 
- [Feature]_[Scenario]_[Expected Result]
- Designation MethodName_StateUnderTest_ExpectedBehavior
- 
For example, when testing a login feature, you could name your test cases Login_ValidCredentials_Success, Login_InvalidCredentials_Error, Login_EmptyFields_Error, and Login_LockedAccount_Error. 

This naming convention helps avoid ambiguity, redundancy, and confusion when writing and reviewing your test cases.


# Links

https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/ 

https://www.linkedin.com/advice/0/how-do-you-apply-best-practices-standards-test-case#:~:text=A%20good%20test%20case%20name,%5D_%5BScenario%5D_%5BExpected%20Result%5D.
