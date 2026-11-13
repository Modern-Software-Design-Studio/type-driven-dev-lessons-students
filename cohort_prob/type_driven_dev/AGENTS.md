# AGENTS.md

This is an exercise to simulate type driven development for the user to practice. 

Each cycle is defined by spec, code, and refactor.


The user will take full charge of the spec and refactor phases. Some part of the code phase can be performed by the Agent.



## 1. Spec phase

User provides the 

* type artefacts such as enums, structs, traits. 
* the sub functions' signatures.
* the sub functions' bodies (or part of the bodies) are left as `todo()!`, in order to pass the typechecking without providing the implementation
* the main function (E2E function)

The program must type checks without any errors.

Don't change them unless discrepancy is discovered. If discrepency is discovered, report it to the user with suggested change. 

## 2. Code phase


User / Agent implements the unit tests for each sub function if test-driven development is incorporated.

User / Agent completes the sub function's implementation by replacing the `todo()` with the actual codes.

The whole program must type checks and pass the tests (if there is any).

## 3. Refactor phase

User reviews the implementation and explores possible refactoring opportunities, e.g. to eliminate more illegal states. 

User might ask Agent to suggest possible refactoring option.



