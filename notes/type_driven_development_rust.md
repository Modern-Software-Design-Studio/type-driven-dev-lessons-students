---
title: '50.057 Type Driven Development in Rust'
author:
- ISTD, SUTD
header-includes:
  - \usepackage{newunicodechar}
  - \newunicodechar{ℕ}{\ensuremath{\mathbb{N}}}
  - \newunicodechar{∀}{\ensuremath{\forall}}
---

# 50.057 Type Driven Development in Rust


## Learning Outcomes 

* Explain the steps involved in Type Driven Development Workflow.
* Apply Type Driven Development in the software project development with LLM.
* Apply Associated type and Peano in Type Driven Development.
* Apply Constant Generics in Type Driven Development.



## Type Driven Development Workflow


According to the literature, type driven development work-flow can be defined as an **iterative** process with the following 3 steps.

1. (Spec) Write the input types and output types of the functions to be implemented.
1. (Code) Define the functions by using the input and output types as a guide.
1. (Refactor)  Refine the type and edit the implementation if needed, i.e. repeat step 1


Let's detail these steps.

### The Spec Phase


During this phase, especially the first iteration, we assume that we have the complete design of the software component that we are trying to develop, which includes,

* Domain Class Diagram, e.g. all the necessarily struct, enum and trait required by the target component 
* Sequence Diagram, e.g. all the call-dependency among the main functions and sub functions


With the above we proceed with the following steps

1. (Re-)define the module and package according to the design.
1. (Re-)define the structs, the enums and the trait.
1. For a new sub-function, leave the function body as undefined, e.g. used `todo!()` in Rust. For sub-function being refactored, make minimum changes just to make it type-checked under the new type annotation, use `todo!()` and hardcoded value as much as possible. 
1. (Very important) implement the end-to-end main function by calling the sub-functions, make sure it type-checks.


**Another important point to take note is that it is often unusual to refactor the end-to-end main functions' type because it is supposed to be interface which was agreed upon.** 

### The Code Phase

Replace the `todo!()`s and hardcoded parts in the sub functions with the actual implementation, which could be carried out by human or AI agents, verify the implementation with type-checking and the test cases.


### The Refactor Phase

Revisit the implementation in the main function with the following questions in mind (by making reference to the design)

1. Is the main flow implemented correctly? 
1. Is there any alternate flow that is not handled?
1. Is there any alternate flow that can be eliminated by type-safe construction? 


These questions sometimes might require more stronger solutions. It might not be sufficient to just setting right type (unless you have full-dependent type, or program verification) and definitely not by running the tests. We will come back to this in the later part when formal verification is discussed. 

In the context of type driven development, we apply our knowledge of logics and the type features offered by the source language. 


Let's consider a few use cases in the context of Rust.


## Associated Type and Peano Type

In this use case, we make use of associated type and peano type to implement a safe `head` function. 

We have come across this example in the [last lesson](./dependent.md).

### Iteration 1

#### Spec 

Assume we are given the design documents, we can start with the following template. 


```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

fn head<T>(xs : List<T>) -> T {
    todo!()
}

fn main() {
    let xs : List<i32> = List::Cons(1, Box::new(List::Nil));
    println!("{}", head(xs));
}
```

We provide the enum type defintion, the type signature of the `head` function and the `main` function with an end-to-end flow. 

We left the function body of `head` as a `todo!()`.


#### Code 

In this phase we work out the actual implementation that satisfy the the type of the function and the test cases (we omitted the test cases for brevity). 

```rust
fn head<T>(xs : List<T>) -> T {
    match xs {
        List::Cons(x, _) => x
    ,   List::Nil => panic!("head is called with an empty list.")
    }
}
```

#### Refactor 


In this phase, we revisit the main function's flow. We realize that `head` function might cause a unhandled run-time error, i.e. there is an alternate flow that is not handled. 

We argue that we should refactor the type of `head` to return an `Option<T>` result. namely 


### Iteration 2 

#### Spec 

We update the type of `head` as follows and adjust the `main` function.

```rust
fn head<T>(xs : List<T>) -> Option<T> {
    match xs {
        List::Cons(x, _) => todo!()
    ,   List::Nil => todo!()   
    }
}

fn main() {
    let xs : List<i32> = List::Cons(1, Box::new(List::Nil));
    match head(xs) {
        Some(x) => println!("{}", x)
    ,   None => println!("empty list")
    } 
}
```

> Question: What is the corresppondence of the type annotation of `head` in logic predicate?


#### Code

We focus on the implementation of the refactored `head` 

```rust
fn head<T>(xs : List<T>) -> Option<T> {
    match xs {
        List::Cons(x, _) => Some(x)
    ,   List::Nil => None  
    }
}
```

#### Refactor

Normally we could stop here. 

However as illusration we go into the 3rd iteration by imposing an additional requirement to eliminate need of `Option<T>` type by making use of the associated type and phantom type.


### Iteration 3 

#### Spec 

We write our types as specification as follows, 

```rust
use std::marker::PhantomData;

struct Zero;

struct Suc<N>(PhantomData<N>);

trait SList {
    type Elem;
    type Size;
}

struct Nil<A>(PhantomData<A>);

impl<A> SList for Nil<A> {
    type Elem = A;
    type Size = Zero;
}

struct Cons<A, Tail: SList<Elem = A>> {
    head: A,
    tail: Tail,
}

impl<A, Tail: SList<Elem = A>> SList for Cons<A, Tail> {
    type Elem = A;
    type Size = Suc<Tail::Size>;
}

fn nil<A>() -> Nil<A> {
    Nil(PhantomData)
}

fn cons<A, Tail: SList<Elem = A>>(head: A, tail: Tail) -> Cons<A, Tail> {
    Cons { head, tail }
}
```

And we update the `head` function's type and leave its body as `todo!()` and we adjusted the `main` function.

```rust
fn head<A, Tail: SList<Elem = A>>(l: Cons<A, Tail>) -> A {
    todo!()
}

fn main() {
    // two : SList (Suc (Suc Zero)) Int
    let two = cons(1, cons(2, nil()));

    // head is total only on non-empty lists, enforced at compile time
    println!("head two = {}", head(two));

    // These do NOT compile — the same type errors as in the notes:
    //
    //   head(nil::<i32>());                     // head Nil
}
```

The code and refactor phases for this iteration are trivial, as the outcome already mentioned in the last class. 



## Constant Generic

Let's consider another scenario. 

Suppose we want to implement a matrix data type with a multiplication operation

### Iteration 1 

#### Spec 

We have the following data type 

```rust
struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}
// and the attached methods for the Matrix struct
impl Matrix {
    fn zero(rows: usize, cols: usize) -> Self {
        todo!()
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        todo!()
    }

    fn set(&mut self, r: usize, c: usize, val: f64) {
        todo!()
    }

    fn new(data: Vec<f64>, rows: usize, cols: usize ) -> Self {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
```

We write the type specification of `matmul` as follows

```rust
fn matmul(
    a: &Matrix, 
    b: &Matrix,
) -> Option<Matrix> {
    todo()!
}
```

With which, we implement the `main` function

```rust
fn main() {
    let a = Matrix::new(
        vec![1.0, 2.0, 3.0,
             4.0, 5.0, 6.0], 2 , 3 
    );

    let b = Matrix::new(
        vec![7.0, 8.0,
             9.0, 10.0,
             11.0, 12.0], 3, 2
    );

    println!("Matrix A (2x3):");
    a.print();

    println!("\nMatrix B (3x2):");
    b.print();

    let oc = matmul(&a, &b);

    match oc {
        Some(c) => {
            println!("\nA * B (2x2):");
            c.print();
        }
    ,   None => println!("dimension mismatch")
    }
}
```

The above type-checks.

#### Code 

As the next step, we replace the `todo!()` with the actual implementation. 


```rust
impl Matrix {
    fn zero(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![0.0; rows * cols], rows, cols }
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r * self.cols + c]
    }

    fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r * self.cols + c] = val;
    }

    fn new(data: Vec<f64>, rows: usize, cols: usize ) -> Self {
        Matrix { data, rows, cols }
    }

    fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }
    
}


fn matmul(
    a: &Matrix, 
    b: &Matrix,
) -> Option<Matrix> {
    let ra = a.rows;
    let ca = a.cols; 
    let rb = b.rows; 
    let cb = b.cols; 
    if ca == rb {
        let mut result = Matrix::zero(ra, cb);
        for i in 0..ra {
            for j in 0..cb {
                for k in 0..ca {
                    result.set(i, j,  result.get(i, j) + a.get(i,k) * b.get(k,j));
                }
            }
        }
        Some(result)
    } else {
        None 
    }
}
```


The detail code of this iteration can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=5e36b78232c2c31a4c2bfcd4b67ed245)

#### Refactor 

The current type of the `matmul` is `(&Matrix , &Matrix) -> Option<Matrix>`. 
In predicate logic, it means for all matrices $a$ and $b$, we can either compute the product or it fails due to dimension mismatch. 

In addition, the `main` function needs to handle the possible `None` value returned by `matmal`. 

In the next iteration, we want to eliminate the need of returning  a `None` in case of dimension mismatch.


### Iteration 2 


#### Spec 

The goal is to change the `matmul` function's type from 
`(&Matrix , &Matrix) -> Option<Matrix>` to 
`(&Matrix , &Matrix) -> Matrix`. But doing so, `matmul` is either throwing a run-time error or implemented as a partial function with a non-exhaustive pattern. 

The informal idea is that if the input matrix $a$'s column dimension is matching with matrix $b$'s row dimension, we are safe! 

Let's try to pen this idea down in predicate logic. We assume there exists a predicate

$$
 matrix(a,x,y)
$$

which checks a given an array $a$ (from the domain $A$), the array's row-dimension $x$, the column-dimension $y$ (from the number domain $N$), forms a valid matrix. This requires some additional auxillary axioms and functions which are omitted here for brevity.

Then we could express our intent as the following logical statement 

$$
\forall x : N. \forall y : N. \forall z : N. \forall a : A. \forall b : A. \\ matrix(a,x,y) \wedge matrix (b, y, z) \implies matrix(matmul(a,b), x, z)
$$

The above logical predicate captures the exact intent of the goal of the refactoring, 
which says, if matrix $a$'s column dimension is matching with matrix $b$'s row dimension, the `matmul` should produce a matrix.  


To encode the above logic predicate in type we need a type features in Rust, *constant generic*.

```rust
struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}
```

In the above revised delcaration, we annotate the `Matrix` struct with `R` and `C` type parameters, with the additional `const` keyword, it enforces that `R` and `C` are immutable.

As a minor adjustment, we use array as the internal data type since the dimension is known. 

We update the accompanion methods 

```rust
impl<const R: usize, const C: usize> Matrix<R, C> {
    fn zero() -> Self {
        todo!()
    }

    fn new(data: [[f64; C]; R]) -> Self {
        todo!()
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        todo!()
    }

    fn set(&mut self, r: usize, c: usize, val: f64) {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
```

We update the signature of `matmul` 

```rust
fn matmul<const R: usize, const K: usize, const C: usize>(
    a: &Matrix<R, K>,
    b: &Matrix<K, C>,
) -> Matrix<R, C> {
    todo!()
}
```

which reflects exactly what was prescribed by the logical predicate defined earlier. 

Finally, we adjust the main function to adjust to the new type signature of `matmul`. We also make sure it type-checks by the Rust compiler.

```rust
fn main() {
    let a = Matrix::<2, 3>::new([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ]);

    let b = Matrix::<3, 2>::new([
        [7.0, 8.0],
        [9.0, 10.0],
        [11.0, 12.0],
    ]);

    println!("Matrix A (2x3):");
    a.print();

    println!("\nMatrix B (3x2):");
    b.print();

    let c = matmul(&a, &b);

    println!("\nA * B (2x2):");
    c.print();

    // This would NOT compile: inner dimensions must match
    // let bad = Matrix::<2, 2>::zero();
    // matmul(&a, &bad); // error: K is 3 for `a` but 2 for `bad`
}
```

#### Code 

In this phase, we could focus on the complettion of the `todo!()`s.

```rust
impl<const R: usize, const C: usize> Matrix<R, C> {
    fn zero() -> Self {
        Matrix { data: [[0.0; C]; R] }
    }

    fn new(data: [[f64; C]; R]) -> Self {
        Matrix { data }
    }

    fn get(&self, r: usize, c: usize) -> f64 {
        self.data[r][c]
    }

    fn set(&mut self, r: usize, c: usize, val: f64) {
        self.data[r][c] = val;
    }

    fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }
}

// Only compiles if inner dimension matches
fn matmul<const R: usize, const K: usize, const C: usize>(
    a: &Matrix<R, K>,
    b: &Matrix<K, C>,
) -> Matrix<R, C> {
    let mut result = Matrix::<R, C>::zero();
    for i in 0..R {
        for j in 0..C {
            for k in 0..K {
                result.data[i][j] += a.data[i][k] * b.data[k][j];
            }
        }
    }
    result
}
```

The full version of iteration can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=70b8290e8d45aa2d9d6e8e2ab0f58096)

#### Refactor

We decide to stop at this iteration. 


## Type State

Let's consider another scenario. 

Suppose we want to implement a `Door` with a state, either locked or unlocked, and the operations `unlock`, `open`, `close` and `lock` on it. A door can only be opened when it is unlocked.

### Iteration 1 

#### Spec 

We have the following data type 

```rust
enum State {
    Locked,
    Unlocked,
}

struct Door {
    state: State,
}
```

and the attached methods for the Door struct

```rust
impl Door {
    fn new() -> Self {
        todo!()
    }

    fn unlock(&mut self) {
        todo!()
    }

    fn open(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn lock(&mut self) {
        todo!()
    }
}
```

With which, we implement the `main` function

```rust
fn main() {
    let mut door = Door::new();
    println!("Door is locked.");

    door.unlock();
    door.open();

    door.close();
    door.open();

    door.lock();
    println!("Door is locked again.");
}
```

The above type-checks.

#### Code 

As the next step, we replace the `todo!()` with the actual implementation. 

```rust
impl Door {
    fn new() -> Self {
        Door { state: State::Locked }
    }

    fn unlock(&mut self) {
        self.state = State::Unlocked;
        println!("Unlocking the door...");
    }

    fn open(&self) {
        match self.state {
            State::Unlocked => println!("Opening the door...")
        ,   State::Locked => panic!("cannot open a locked door.")
        }
    }

    fn close(&self) {
        println!("Closing the door...");
    }

    fn lock(&mut self) {
        self.state = State::Locked;
        println!("Locking the door...");
    }
}
```

#### Refactor 

The current type of the `open` method is `fn(&Door)`. 
In predicate logic, it means for all doors $d$, we can open it, regardless of its state. 

In reality, opening a door that is locked is an error, which can only be detected at run-time, i.e. there is an alternate flow that is not handled (the `panic!` in the above implementation). 

In the next iteration, we want to eliminate the need of the run-time check by making the state of the door part of its type.


### Iteration 2 


#### Spec 

The goal is to change the type of the door from `Door` to `Door<State>`, where the state is encoded as a type parameter, so that `open` is only available on a door of type `Door<Unlocked>`.

The informal idea is that a locked door and an unlocked door are different types, so that the call to `open` on a locked door is a type error. 

Let's try to pen this idea down in predicate logic. We assume there exists a predicate

$$
 unlocked(d)
$$

which checks a given door $d$ (from the domain $D$) is in the unlocked state. Then we could express our intent as the following logical statement 

$$
\forall d : D. open(d) \implies unlocked(d)
$$

The above logical predicate captures the exact intent of the goal of the refactoring, 
which says, the `open` operation is applied to a door only when the door is in the unlocked state.

To encode the above logic predicate in type we need to lift the value-level state to the type level, a pattern known as *type state*.

We first introduce two marker types that carry no run-time data

```rust
struct Locked;
struct Unlocked;
```

and annotate the `Door` struct with a `State` type parameter. Since `State` does not appear in any data field, we use a `PhantomData` field to tell the compiler that the type depends on `State` without storing it.

```rust
struct Door<State> {
    _state: std::marker::PhantomData<State>,
}
```

We now define the methods in two separate impl blocks, one for each state. The state transitions, `unlock` and `lock`, consume the door and produce a door of the new state, while the other operations preserve the state.

```rust
impl Door<Locked> {
    fn new() -> Self {
        todo!()
    }

    fn unlock(self) -> Door<Unlocked> {
        todo!()
    }
}

impl Door<Unlocked> {
    fn open(&self) {
        todo!()
    }

    fn close(self) -> Door<Unlocked> {
        todo!()
    }

    fn lock(self) -> Door<Locked> {
        todo!()
    }
}
```

Finally, we adjust the `main` function to the new type signatures. Note that `unlock`, `close` and `lock` consume the door, so we rebind the variable to the returned door.

```rust
fn main() {
    let door = Door::<Locked>::new();
    println!("Door is locked.");

    // door.open(); // Would NOT compile: door is Locked, open() requires Unlocked

    let door = door.unlock();
    door.open();

    let door = door.close();
    door.open();

    let _door = door.lock();
    println!("Door is locked again.");

    // door.open(); // Would NOT compile: door is Locked again
}
```

The above type-checks. The call to `open` on a locked door is now a compile-time type error instead of a run-time error.

#### Code 

In this phase, we could focus on the completion of the `todo!()`s.

```rust
impl Door<Locked> {
    fn new() -> Self {
        Door { _state: std::marker::PhantomData }
    }

    fn unlock(self) -> Door<Unlocked> {
        println!("Unlocking the door...");
        Door { _state: std::marker::PhantomData }
    }
}

impl Door<Unlocked> {
    fn open(&self) {
        println!("Opening the door...");
    }

    fn close(self) -> Door<Unlocked> {
        println!("Closing the door...");
        self
    }

    fn lock(self) -> Door<Locked> {
        println!("Locking the door...");
        Door { _state: std::marker::PhantomData }
    }
}
```

The full code of this iteration can be found in [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=73599ed53cdc56aa4bd30e86c3d92db2).

#### Refactor

The illegal transition, i.e. calling `open` on a locked door, is now untypeable: `Door<Locked>` and `Door<Unlocked>` are different types, and `open` is only defined on `Door<Unlocked>`.

We decide to stop at this iteration.


## Type Driven Development vs Test Driven Development


Recall that the principle of the type driven development is to identify the failing states and make them untypeable via static type analysis (type checking).

Recall from the even earlier classes, we learned about Test Driven Development where the focus is to think about the interface design via writing the tests first. 


The following table summerizes the comparison


|                        | Type DD | Test DD   |
|------------------------|---------|-----------|
| Spec Representation    | Types   | Test cases|
| Design Focus           | What (run-time) failures should be eliminated | What should work | 
| Interface orientation  | Via datatype and function signature           | Via test cases | 
| Feedback Mechanism     | Compiler with advance type system             | Expressive Test Framework | 
| Cost                   | Very hard to follow, but the feedback is cheaper   | Easier to follow, but the feedback requires running the tests | 


### Limitation

We use the famous quote from Edsger W. Dijkstra

"Program testing can be used to show the presence of bugs, but never to show their absence!"

Which highlights the limitation of Test Driven Development. With type driven development, we are able to 
use type system to eliminate certain type of bugs. 

Can we just go with type driven development without tests? Unfortunately no. As stated by the Rice Theorem

"All non-trivial semantic properties of programs are undecidable."

Which highlights the limitation of Type Driven Development as type is a form of static semantic analysis, which is 
undecidable in general. 

A combination of Type Driven Development with Test Driven Development is praticed in some software engineering teams, for instance, during the "Code" phase of the Type Driven Development iteration, we can apply a test driven development approach to use the test cases to drive the code development. 



## Type Driven Development with AI Agents

One common practice of AI assited devepoment is to engage the AI agents during the "Code" phase; while the human engineers must actively driving the "Spec" phase and "Refactor" phase. 

By setting ground rules in the `AGENTS.md` file helps to remind the agents not to override human's intention (the type annotation in all situation). 


## Data Parsing and Validation at the Boundary

Type driven development makes the invalid states unrepresentable *within* the domain. However, a real system receives data from outside the reach of the type system: the JSON payloads of the API calls, the rows of the database, the strings of the HTTP requests. Such external data can not be type-checked at compile time, and the boundary of the system is where the type driven workflow meets its limit. We need to handle this boundary deliberately. 

### The boundary is where the `Result` legitimately remains 

Recall the question from the Refactor phase: *is there any alternate flow that can be eliminated by type-safe construction?* For the internal states, we answered "yes" in each of the earlier iterations, e.g. the `Option` of the empty list, the dimension mismatch of the `matmul`, and the state of the door. For the *external* data, the answer is "no". A malformed JSON string, or a `NULL` database column, is a genuine run-time alternate flow: it can not be made unrepresentable, because its representation is precisely the untrusted input. 

So the type driven stance is not to eliminate the `Result` at the boundary, but to **concentrate** it there: all the fallible parsing and validation happens in one layer, and the rest of the program operates only on the validated values. 

### Raw types at the edge, validated types in the core 

The concrete pattern is to introduce two families of types with a wall in between:

* **Wire types**: the types produced directly from the external representation, e.g. the `serde_json::Value`, the `String`, or a struct that mirrors the shape of the JSON or the database row. Nothing about a wire type is guaranteed. 
* **Domain types**: newtypes (a struct with a single field, e.g. `struct Email(String);`) whose invariants hold *by construction*, i.e. they can only be produced by a validating constructor. 

For instance, suppose we receive a user's email address from an API call. We define the domain type as follows 

```rust
struct Email(String);

impl Email {
    fn parse(address: &str) -> Result<Self, ValidationError> {
        if !address.contains('@') {
            return Err(ValidationError::InvalidEmail);
        }
        Ok(Email(address.to_owned()))
    }
}
```

Assume there exists a predicate

$$
 valid(e)
$$

which checks a given address $e$ (from the domain $S$ of the strings) is a valid email address. Then the type of the constructor states

$$
\forall e : S. \forall x : S. \; parse(e) = Ok(x) \implies valid(x)
$$

which says, every `Email` value produced by the constructor satisfies the validity predicate. This is exactly the property we exploited in the earlier iterations: once a value has crossed the boundary, no function inside the domain needs to validate it again. The domain functions take the `Email` type instead of the `String` 

```rust
fn send_greeting(to: &Email) {
    // no validation needed here: `to` is known-valid by its type
}
```

Writing down the logic predicate representation of the boundary parseing behavior not only helps us to nail the type spec but also supports the verification process which will be dicusssed in the following classes. 

### Spec, Code, and Refactor at the boundary 

Let's apply the workflow to the boundary layer. For each iteration (due to the domain type refactoring)

* **Spec.** Write (or re-check) the boundary signatures, which is the agreed interface and is rarely refactored, like the `main` function (because `main` function here or public interface of this domain is expected to be invoked by external call-sites, which might not be trusted). The domain types are newtypes with private fields and a validating constructor, e.g. `fn parse_user(raw: &str) -> Result<User, BoundaryError>`. This is the specification we hand over to the AI agent or to a team member. 
* **Code.** Implement (or update) the parsing, e.g. deserializing the JSON into the wire types, and the validation, where each invariant is checked exactly once. This is the one place where test driven development is indispensable: by the Rice Theorem, the validity of the external data is undecidable at compile time, so the malformed inputs, e.g. missing fields, out-of-range values, unexpected types, must be covered by the test cases. 
* **Refactor.** Ask whether any residual `Option` or `Result` *inside* the domain can be pushed further inwards: a range already encoded by a primitive, e.g. a port as a `u16`, needs no newtype; a lifecycle invariant can be lifted into a type state; a fixed size can be a constant generic. The boundary itself, however, keeps its `Result`, because that is the wall. 

### Rules of thumb 

1. Validate once, at the boundary. Never re-validate an invariant inside the domain, the type already states it. 
1. Keep the wire types and the domain types separate, so that a change of the external representation, e.g. a field rename of the API or the schema, breaks only the boundary layer. 
1. Fail fast, and distinguish the errors: a *parse* failure, i.e. a malformed representation, is different from a *validation* failure, i.e. a well-formed but illegal value, and both should surface immediately at the boundary. 
1. For the database calls, treat every row mapping as a boundary: a missing column or a `NULL` is a parse failure, and a value that violates an invariant is a validation failure. 
1. For the API clients, the client exposes the domain types to the rest of the program, while the wire types remain private to the client module. 


## Further Reading 

* https://www.ruggero.io/blog/rust_type_driven_development_guide/
* https://www.manning.com/books/type-driven-development-with-idris
