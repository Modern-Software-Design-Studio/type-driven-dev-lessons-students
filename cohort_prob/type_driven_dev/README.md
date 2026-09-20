# Type Driven Development Exercises: Sorted List Invariants

These exercises follow the type driven development workflow described in
`notes/type_driven_development_rust.md`. You will refactor a plain list type,
in which the "sorted" invariant is only a runtime convention, into a typestate
design in which the invariant is part of the type, so that illegal operations
become untypeable.

The final answer can be found in `examples/05_sorted_list`.

Work in this directory:

```bash
cargo run
```

## Iteration 1 - The Starting Point

`src/main.rs` contains a list type with the following operations:

| Operation | Intended contract |
|---|---|
| `insert` | if the list is sorted, the result stays sorted |
| `merge` | if both lists are sorted, the result is sorted |
| `concat` | appends one list after the other |
| `push` | appends a value at the end |
| `sort` | sorts the list, restoring the invariant |

Run the program and observe the output.

## Exercise 1: Refactor - Identify the Failing States

1. State the sorted invariant as a predicate $sorted(a)$ over lists $a$, i.e. define the predicate $sorted(a)$ by completting the following predicate 
$$
\forall a. sorted(a) \iff ... 
$$
2. In the output of `main`, identify the results that violate the intended
   contracts in the table above. For each, explain which precondition was
   silently broken.
3. Why is this kind of failure more dangerous than a run-time `panic!`?
   (Hint: compare with the `head` function in the notes.)
4. For each of the five operations, state whether it (i) requires sorted
   input, (ii) guarantees sorted output, or (iii) both / neither.

## Exercise 2: Spec - Derive the Refined Types (Iteration 2)

We want to make the failing states untypeable: `merge` and `insert` should
only be callable on sorted lists, and an operation that may break the
invariant should return a type that records that the invariant no longer
holds.

Assume the predicate

$$
sorted(a)
$$

which holds when the list $a$ is in non-decreasing order. The intended
contracts from iteration 1 can then be written as

$$
\forall a.\ \forall v.\ sorted(a) \implies sorted(insert(a, v))
$$

$$
\forall a.\ \forall b.\ sorted(a) \wedge sorted(b) \implies sorted(merge(a, b))
$$

$$
\forall a.\ sorted(sort(a))
$$

and for `concat` no such guarantee exists:

$$
\exists a.\ \exists b.\ sorted(a) \wedge sorted(b) \wedge \neg sorted(concat(a, b))
$$

(for example, $[1,3,5]$ and $[2,4,6]$).

Following the typestate pattern from the Type State section of the notes,
answer the following.

1. Define the marker types and the new `List` struct, with its state type
   parameter and phantom field. Hints: it should look something liek the following 

```rust
struct Sorted;
struct Unsorted;

struct List<State, T> {
    data: Vec<T>,
    _state: std::marker::PhantomData<State>,
}
```

2. For each operation below, write the refactored type signature: which impl
   block it belongs to, the receiver type, and the return type.

| Operation | Refactored signature | Justification |
|---|---|---|
| `new` | | |
| `insert` | | |
| `merge` | | |
| `concat` | | |
| `sort` | | |
| `push` | | |
| `forget_sorted` | | |

3. Justify each row with one of the predicates above. For `new`, think about
   what $sorted(\_)$ says about the empty list. For `forget_sorted`, think
   about it as the explicit escape hatch that gives up the guarantee.
4. State the trait bounds on `T` for each impl block, and explain why the two
   blocks need different bounds. Hints:

    * For the elements of type `T` to be comparable, what trait bound should `T` have?
    * For the elements to be copiable into the list via the `insert` and `push`, what trait bound should `T` have?
    * For the list and its elements to be printable, what trait bound should `T` have? 
    * For `sort` to call `Vec::sort`, what trait bound shoult `T` have?


After deriving the signatures, check them against the scaffold in Exercise 3.

## Exercise 3: Code - Implement the Refined Type (Iteration 2)

Replace the content of `src/main.rs` with the following scaffold. It is the
outcome of the Spec phase: the types are the specification, the bodies are
`todo!()`.

```rust
use std::cmp::{Ord, PartialOrd};

struct Sorted;
struct Unsorted;

struct List<State, T> {
    data: Vec<T>,
    _state: std::marker::PhantomData<State>,
}

impl<T: Clone + std::fmt::Debug + PartialOrd> List<Sorted, T> {
    fn new() -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn from_single(value: T) -> Self {
        todo!()
    }

    // Maintain the sorted invariant.
    fn insert(self, value: T) -> Self {
        todo!()
    }

    // Both inputs are sorted, so the output is sorted.
    fn merge(self, other: Self) -> Self {
        todo!()
    }

    // Two sorted lists concatenated are not necessarily sorted.
    fn concat(self, other: Self) -> List<Unsorted, T> {
        todo!()
    }

    // Give up the sorted guarantee explicitly.
    fn forget_sorted(self) -> List<Unsorted, T> {
        todo!()
    }

    fn print(&self, label: &str) {
        todo!()
    }
}

impl<T: Clone + std::fmt::Debug + Ord> List<Unsorted, T> {
    // Restore the sorted invariant.
    fn sort(self) -> List<Sorted, T> {
        todo!()
    }

    fn push(self, value: T) -> Self {
        todo!()
    }

    fn print(&self, label: &str) {
        todo!()
    }
}

fn main() {
    // --- Insert into sorted list ---
    println!("=== Insert (preserves sorted) ===");
    let list = List::<Sorted, i32>::new()
        .insert(5)
        .insert(2)
        .insert(8)
        .insert(1)
        .insert(3);
    list.print("after inserting 5,2,8,1,3");

    // --- Merge two sorted lists ---
    println!("\n=== Merge (preserves sorted) ===");
    let left = List::<Sorted, i32>::new().insert(1).insert(3).insert(5);
    let right = List::<Sorted, i32>::new().insert(2).insert(4).insert(6);
    left.print("left");
    right.print("right");

    let merged = List::<Sorted, i32>::new()
        .insert(1)
        .insert(3)
        .insert(5)
        .merge(List::<Sorted, i32>::new().insert(2).insert(4).insert(6));
    merged.print("merged");

    // --- Concat two sorted lists (NOT sorted) ---
    println!("\n=== Concat (loses sorted invariant) ===");
    let a = List::<Sorted, i32>::new().insert(1).insert(3).insert(5);
    let b = List::<Sorted, i32>::new().insert(2).insert(4).insert(6);
    a.print("a");
    b.print("b");

    let concatted = List::<Sorted, i32>::new()
        .insert(1)
        .insert(3)
        .insert(5)
        .concat(List::<Sorted, i32>::new().insert(2).insert(4).insert(6));
    concatted.print("concat(a, b) - not sorted!");

    // --- Sort to restore invariant ---
    println!("\n=== Sort (restores sorted) ===");
    let sorted_again = concatted.sort();
    sorted_again.print("after sort");

    // --- Type-level enforcement ---
    println!("\n=== Type-level enforcement ===");
    let sorted = List::<Sorted, i32>::new().insert(10).insert(20);

    // sorted.push(5);           // WON'T compile: push() doesn't exist on List<Sorted>
    // sorted.concat(sorted2);   // Returns Unsorted, not Sorted

    let unsorted = sorted.forget_sorted().push(5);
    unsorted.print("after forget_sorted + push(5)");

    // unsorted.merge(other);    // WON'T compile: merge() only exists on List<Sorted>
    // unsorted.insert(3);       // WON'T compile: insert() only exists on List<Sorted>
}
```

The `main` function is the agreed end-to-end interface; do not change it.
Your job is to make it type-check and run.

Implement the following **by hand**; each is a direct consequence of the type
signatures or of the invariant:

- `new`, `from_single`, `push`, `sort`, `forget_sorted`
- `insert` - must maintain the sorted invariant; find the position at which
  `value` belongs
- `concat` - note carefully what the return type says
- `print` - adapt from iteration 1

For `merge`, you may let an **AI agent** generate the body. The type
signature is the specification you give the agent:

- Give the agent the signature
  `fn merge(self, other: List<Sorted, T>) -> List<Sorted, T>` and the
  invariant "both inputs are sorted; the output must be sorted".
- Verify the generated code: it type-checks, `cargo run` prints the expected
  result, and the invariant holds (test with at least two non-trivial sorted
  inputs).
- In your write-up, paste the prompt you used and the code the agent
  generated, and explain why the type signature is sufficient as a
  specification.


#### Expected Output

When you are done, `cargo run` should print the expected output as follows

```
=== Insert (preserves sorted) ===
  after inserting 5,2,8,1,3 (sorted): [1, 2, 3, 5, 8]

=== Merge (preserves sorted) ===
  left (sorted): [1, 3, 5]
  right (sorted): [2, 4, 6]
  merged (sorted): [1, 2, 3, 4, 5, 6]

=== Concat (loses sorted invariant) ===
  a (sorted): [1, 3, 5]
  b (sorted): [2, 4, 6]
  concat(a, b) - not sorted! (unsorted): [1, 3, 5, 2, 4, 6]

=== Sort (restores sorted) ===
  after sort (sorted): [1, 2, 3, 4, 5, 6]

=== Type-level enforcement ===
  after forget_sorted + push(5) (unsorted): [10, 20, 5]
```

## Exercise 4: Refactor - The Illegal States are Untypeable

1. Take a `List<Unsorted, i32>` value (for example, the result of `concat`).
   Show the compiler error for each of `unsorted.merge(other)` and
   `unsorted.insert(3)`, and state which predicate each error enforces.
2. Why must `concat` return `List<Unsorted, T>` rather than
   `List<Sorted, T>`? Under what additional condition on the two inputs could
   it return `List<Sorted, T>`, and how could that condition be checked at
   run time?
3. `forget_sorted` explicitly discards the sorted guarantee. Is providing it
   a contradiction to the goal of the refactoring? Compare it with the silent
   loss of the invariant in iteration 1.
4. (Optional) Could `push` be defined on `List<Sorted, T>`? What would its
   return type have to be, and why?
5. (Optional) Where would a `from_single` constructor (a list with a single
   element) belong, and why?
