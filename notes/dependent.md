---
title: '50.057 Dependent Type and Curry-Howard Correspondence'
author:
- ISTD, SUTD
header-includes:
  - \usepackage{newunicodechar}
  - \newunicodechar{ℕ}{\ensuremath{\mathbb{N}}}
  - \newunicodechar{∀}{\ensuremath{\forall}}
---

# 50.057 Dependent Type and Curry-Howard Correspondence


## Learning Outcomes

* Explain the Curry-Howard Correspondence
* Explain dependent type
* Explain Phantom type in Rust 
* Explain Associated type in Rust
* Use Phantom type and Associated type in Rust to encode some dependent type


## Value versus Type

In computer programming languaes, a *value* denotes a piece of information that is stored or manipulated during runtime.


#### Example 1

For example, in the following program variable `x` is first initialized as an integer value `1`.
The value `1` is a datum to be sent to variable, then is further computed in the next statement. 

```rust
fn main() {
    let x : i32  = 1;
    println!("{:?}", x+1)
}
```

The annotation `i32` on the hand, is called a *type*. In this case it dentoes a 32-bit integer type. A type in programming language denotes a set of values.  In many modern programming 
language, the type should have no impact to the run-time. It serves a compile-time assertion that 
the compiler must enforce before generating the target code.  It also helps us (the programmers) to 
better comprehend the intention of the code, for example, we know that `x` must be integer, it can't be a string. 


Base on the same structure, we could define compound values and compound types. 

```rust
struct Point {
    xcoord : i32 , 
    ycoord : i32 
}

fn main() {
    let p : Point = Point {
        xcoord : 10 , 
        ycoord : 30 
    }
    // ... do something with p
}
```

In the `main` function above, we define a compound data value `p` comprise of two values `10` as the x coordinate `30` as the y coordinate. On the type level, via the `struct` declaration, we define a new type `Point` from two `i32` types. A `Point` type denotes a set of possible x and y coordinate value pairs. 


## Proposition as Type, Proof as Code 

It's a direct relationship between logic and programming discovered and formalized by Haskell Curry and William A. Howard in the 1940-1970. It is also known as *Curry-Howard Correspondence*.

#### Example 2

For instance, we claim that *all SUTD students are smart*.

In predicate logic, we write 

$$
\forall x. sutdent(x) \implies smart(x)
$$

> Note: "sutdent" is a neologism for "SUTD student".

Quick recap: What is 

* $\forall$?
* $x$?
* $sutdent(\_)$?
* $\implies$ ? 
* $smart(\_)$ ? 

In Rust, we can implement the above as 

```rust
struct Sutdent {
    first : String, 
    last : String, 
    adminno : String 
}

trait Smart {
    fn knows_logic(&self) -> String ;
}

impl Smart for Sutdent {
    fn knows_logic(&self) -> String {
        format!("{} attends MSDS.", self.first)
    }
}

fn main() {
    let abby : Sutdent = Sutdent {
        first : String::from("Abby") , 
        last : String::from("Sim") , 
        adminno : String::from("1009394") 
    };
    print!("{}",abby.knows_logic());
}
```
The full version of the code can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=98fef19e006fdc1b651bace60d8ab46e).

In the above Rust program, we define a compound type `Sutdent` which captures the essential information of a student from SUTD. 

We define a trait `Smart` to constraint for a person (or a data instance in the programming sense) being smart is to know logic.

In the trait implementation of `Smart for Sudent`, we supply a concrete implementation of the obligation `knows_logic`. 

What we are trying to illustrate in this example is that 

* by representing the `Sutdent` as a type, `Smart` as a trait (AKA the type) and 
* by implementing `Smart for Sutdent` trait instance with a concrete function (AKA the program) , 
* we are "proving" the proposition that *all SUTD students are smart*. 

#### Example 3 

Let's consider another approach of proving *all SUTD students are smart* via a more language-agnostic approach.

```rust
// struct Sutdent is same as example 2
struct Smart {
    first : String,
    last : String, 
    knows_logic : fn(&Smart) -> String 
}


fn attends_msds(s : Sutdent) -> Smart { 
    Smart {
        first : s.first, 
        last : s.last, 
        knows_logic : |sp : &Smart| -> String {
            format!("{} attends MSDS.", sp.first)
        }
    }
}

fn main() {
    let abby : Sutdent = Sutdent {
        first : String::from("Abby") , 
        last : String::from("Sim") , 
        adminno : String::from("1009394") 
    };
    let smart_abby : Smart = attends_msds(abby);
    print!("{}",(smart_abby.knows_logic)(&smart_abby));
}
```

The full version of the code can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=71dac7ea8784113d2592dc197c89604e).


In this version, we represent the `Smart` predicate as a compound type, the function `attends_msds` has the exact type `Sutdent -> Smart` which reads turns a SUTD student into a smart person, or in logic sense, attending MSDS is a proof (or a way) to turn any SUTD student into smart one. The function type constructor `->` in Rust is just like the logical implication $\implies$.

> BTW, the above is also how Rust's trait is implemented internally. This method implementing trait as a compound type, a trait implementation (instance) as a function is called *dictionary passing*.



### Curry-Howard Correspondence, what's the big deal?

Curry-Howard correspondence is a great idea, esp during the current moment with a lot of technology (and econonmic) shift in the software engineering business. 


There are theorical (and philosophical) benefit and (for sure) practical benefit of applying Curry-Howard Correspondence. 

#### Empirical Application of Curry-Howard Correspondence

There are at least two major directions of application Curry-Howard Correspondence in SWE. 


1. Exploit the correspondence to "strengthen" the software quality assurance process via formal verification. For instance, we can verify the algorithm implemented in the software system is correct *mathematically* by writing the proof (the proof code) in a proof assistant language, e.g. such as Lean, Rocq and Agda. We will study this in a few weeks time. The proof assitant languages have a stronger type system that allows us to express stronger (clearer) properties using Dependent type.
1. Exploit the correspondence to "harden" the development process to make it to be correct in the first attempt. For instance, we can exploit advanced typing mechanism in the main language such as Rust, TypeScript and etc to "incribe" the correct intention in type annotation (since types are logic proposition). If the implemention (i.e. the code) is typechecked by the compiler, the correctness w.r.t to the intent (in the type annotation) is entailed logically. In the age of LLMs, this means that we should write specification on the level of type (propositional predicates), and let the agent generate the codes (the proof).


For the rest of this course, we will first study the 2nd application as type-driven development. In a few weeks, we will look into formal verification.


#### Theorical and Philosophical Application of Curry-Howard Correpondence

Back in the 19th century, mathematicians and logicians were thinking about what computation should be and how they can be generalized. David Hilbert in 1920s, was conjecturing that there could be a *universal* algorithm that solves all problems, AKA Entscheidungsproblem. 

The conjecture was disproved by two concurrent findings.

1. Kurt Gödel (1931) - the theorem of incompleteness, which says that in any consistent mathematical system, there are true statements that can never be proven within that system. 
1. Alan Turing and Alonzo Church (1936) - the halting problem, which showed that a universal algorithm to solve all problems cannot exist.

In 60-70s, Curry-Howard Correspondence bridged these two findings into a canonical form.

This sounds very discouraging at first. However this opens up many research and development opportunities for computer scientists and software engineers. Even in the age of LLMs, we are not afraid of the machines are capable generating codes given tbe prompt, because finding a provable theorem is not an easy task, and thanks to Curry-Howard Correspondence, finding a implementable and correct design is a hard problem.

## Dependent Type - the first encounter

So far, all the programs we have seen have a clear boundary between 
A dependent type is a type whose definition depends on a value instead of other types.

Consider the following Rust program 

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

fn head<T>(xs : List<T>) -> T {
    match xs {
        List::Cons(x, _) => x
    ,   List::Nil => panic!("head is called with an empty list.")
    }
}

fn main() {
    let xs : List<i32> = List::Cons(1, Box::new(List::Nil));
    println!("{}", head(xs));
}
```

In the above program, we define a dynamically linked list using an enum type. 
We want to define a function `head` which extracts the first element from the linked list 
if the list is not empty; otherwise a run-time error is triggered. 

What if we want to "fix" the above program so that there is no run-time error being thrown.
One may argue that we should change the return type of `head` into an option type `Option<T>`.
This is definitely a possible fix.

However let's say we don't want to take this route. Instead we consider limiting the usage of `head` to be applied to only non-empty lists. To do so we need to augment the list datatype with its size. 



Let's formalize our intention using predicate logic. In this case, we extend the peano system we have seen 
in the last section. Hence besides $zero$ term and $suc(\_)$ function, we introduce the following to encode list

- $nil$ - the empty list term
- $cons(\_, \_)$ - the non-empty list constructor function. 

The `list` predicate can be defined recursively as follows

$$
list(nil) 
$$

$$
\forall x. \forall xs. list(cons(x, xs)) \iff list(xs) 
$$

Implicitly, we assume that there exist two sub domains of $Obj$, namely the peano numbers $N$ and lists $L$. The domain of variable $x$ is $Obj$ and the domain of variable $xs$ is $L$.


To measure the length of a list, we define 

$$
eq(length(nil), zero)
$$

and

$$
\forall n. \forall xs .eq(length(cons(x,xs)), suc(n)) \iff eq(length(xs), n)
$$

for readability, we include $n$ as a variable for numbers, and we sometimes use $\_==\_$ instead of $eq(\_,\_)$.

$$
length(nil) == zero
$$

and

$$
\forall n:N. \forall xs:L . length(cons(x,xs) == suc(n)) \iff (length(xs) == n )
$$

The property we want to ensure can be expressed as follows,

$$
\forall n.\ \forall xs.\ \Big( list(xs) \wedge length(xs) == suc(n) \implies \exists x.\ head(xs) == x \Big)
$$

#### A glimpse into the future : Agda
In a language that supports dependent type fully, such as Agda, we would implement it as follows by following Curry-Howard Coorespondence 

```agda
data SList ( A : Set ) : ℕ → Set  where
  nil : SList A zero
  cons :  { n : ℕ } 
        → ( x : A ) 
        → ( xs : (SList A n) ) 
        → SList A (suc n)

head : ∀ { A : Set } { n : ℕ } → SList A (suc n) → A
head  (cons {n} x xs) = x
```

This is our first time seeing Agda code in this module. Let's have a quick tour of the syntax.

1. `Set` is a builtin keyword, denoting the type (universe) of all types. 
1. The `data` keyword in Agda introduces an algebraic datatype, (like `enum` in Rust).
1. The `A` is the generic of the `SList` to represent the underlying element type (we don't need this in the predicate logic). `A` is having the scope of the entire data type definition, i.e. the usage of `A` in `nil` and `cons` is referring to this `A`.
1. The `ℕ` is the peano numeric type defined in Agda's standard library. It is placed after the `:` to indicate that it is an *index* type, i.e. `SList` depends on the values of `ℕ`. 
1. The `nil` defines an empty list, with size `zero`. 
1. The `cons` constructor defines a non-empty list, with size `suc n` where `n` is the size of its tail `xs`.

Conveniently and concisely, we combine the construction of the list and its size together into the data type, since the list data and its size are both defined recursively. The advantage is that we don't need to define the `length` function (though we could) in this example. 

Note that the `SList` data type here is **constructed** using a value of `ℕ` type, e.g. `zero`, `n` and `suc n` are appearing in the types of `nil` and `cons`. Hence `SList` is a dependent type.

Now we can translate the property that we want in predicate logic into type of the `head` function in Agda. Following the same Curry-Howard mapping we have used before ($\forall$ is a quantifier, $\to$ is an implication), each fragment of the type reads as:

| Type fragment | Predicate-logic reading |
|---|---|
| `∀ { A : Set }` | $\forall A$ (for any element type) |
| `∀ { n : ℕ }` | $\forall n \in \mathbb{N}$ |
| `SList A (suc n)` (the domain) | $list(xs) \wedge length(xs) == suc(n)$ : "$xs$ is a list of $A$ with a *positive* length" |
| `→` | $\Rightarrow$ |
| `A` (the codomain) | $\exists x : A.\ x == head(xs)$ — the result is a witness of type $A$ |


Three observations worth making:

1. `suc n` is precisely our *non-emptiness* predicate, since $\exists n.\ length(xs) == suc(n) \iff \neg (xs == nil)$. .
1. The axiom $\neg head(nil)$ is implied. Since `nil : SList A zero` while the domain of `head` requires `SList A (suc n)`, there is simply no term that applies `head` to `nil`. In the proof reading, we do not *prove* $\neg head(nil)$; we make it impossible to even *state* a counterexample. This is the "illegal states are unrepresentable" idea, and it is exactly what a plain type system (or an `Option<T>` return type) cannot provide.
1. The connection between the return type and the predicate logic, `A` (the codomain) vs $\exists x : A.\ x == head(xs)$ is implicit, although we could make it more explicit in Agda, which just makes the body of the `head` function unnecessarily complex. We can come back to this point in a later section.


We will come back to Agda when we discuss formal verification in a later section. 


#### Fake it till you make it

Rust does not support dependent type. We could exploit the advanced type features to mimic dependent type. 

We first need to define the Peano number system. 

```rust
use std::marker::PhantomData;

struct Zero;
struct Suc<N>(PhantomData<N>);
```

In the above we create two singleton struct types `Zero` and `Suc` so that we can have numbers on the type level.  For example, we could have a type `1` on the level of Rust type system as `Suc Zero`. 

Note that we need to make use of a the `PhantomData` package, becaues we want `Suc` to be a type constructor (i.e. it takes another type `N` as input and returns a new type). Since it is a struct, if we omit the `PhantomData<N>`, Rust will complain that `N` is not used in `Suc` (on the value level, though we don't care). 

A *phantom* type to a data type is a type parameter is not mentioned in the data type's data constructor. 

Alternatively, we could use enum without the need of phantom type.

```rust
enum Suc<N> {
    Suc(N)
}
```


Next we need to define our size list `SList` as a trait.

```rust
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
```

Note that we "lift" the data constructors `Nil` and `Cons` into the type level and declare that they are instances of the `SList` trait. 

There is another new type feature that we use here, *associated type*. 

In a Rust, the `type` keyword is overloaded. When it is used outside of a trait, it defines a type alias; when it is used inside a trait, it introduces an associated type. We can think of an associated type is a type embeded inside a trait which get instantiated when the trait is implemented. 

For instance in the `Nil` implementation of `SList`, we bind the associated type `Elem` to `A` and `Size` to `Zero` (the `Zero` type). Likewise, in the `Cons` case, we bind `Size` to `Suc<Tail::Size>` since the type argument `Tail` is a trait instance of `SList<Elem = A>`. (Note `Tail` is just another generic like `A`, except that it is bounded.) By doing so, we can enforce that the property that an `SList` created by `Cons` must be non-zero in size. 

So the main idea here is to make the **illegal states become unconstructable and untypeable**.


```rust
fn nil<A>() -> Nil<A> {
    Nil(PhantomData)
}

fn cons<A, Tail: SList<Elem = A>>(head: A, tail: Tail) -> Cons<A, Tail> {
    Cons { head, tail }
}


fn head<A, Tail: SList<Elem = A>>(l: Cons<A, Tail>) -> A {
    l.head
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

Lastly we define some short-hand functions for construction and the `head` function.  Note that the commented call to `head` with an empty list won't compile.

The full program can be found [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e3b3119b06dec4cba118036793367639)


### Further Reading 

* https://www.ruggero.io/blog/rust_type_driven_development_guide/
* https://plato.stanford.edu/entries/type-theory-intuitionistic
