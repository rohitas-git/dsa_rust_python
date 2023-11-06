
# System Design

Large scale distrubuted system 

Got a problem / bussiness req

- Define the requirements from user's perspective
  - Identify the core concerns and features 
    - Can be observed from Product Requirement Document 
  - Reduce the features to data definitions
  - Mapped to objects
  - Mapped into database

Req -> Abstract Concepts/Features -> Data defs -> Objects -> Database

Once you have defined the data you need to store, you need endpoints through which this data can be manipulated or queried

When creating designs or code, make sure that none of the services fail if there is an outage. 

Engineering Requirements:
- Having multiple points of failures
- Having extensibility (how easy it change the solution)

Problem with writing code, which is highly coupled with the feature:
- Whenever there is a changing requirement, need to put alot of effort to redesign, test, deploy that code again

So,
- Build a system that can scale and extend as and when requirements change

Finally, Design needs to be tested
- Load testing
- Capacity estimation
- Before getting into the code

Till now,
- create a extensible solution
- test the solution


## High level design
Make a blueprint of the system

## Low level design
Take small chunks of the system and try to elaborate and code each chunk
Take certain function of these services and code it out

Going into much more depth rather than breadth

What are the actions that a user can perform?
