# Type Driven Development Homework: Gradient Descent with Compile-Time Dimensions

These exercises follow the type driven development workflow described in
the note. You will refactor a gradient descent
implementation, in which vector and matrix dimensions are only checked at run
time, into a design that encodes the dimensions in the types using const
generics, so that a dimension mismatch becomes untypeable.


Work in this directory:

```bash
cargo run
```

## Iteration 1 - The Starting Point

`src/main.rs` contains a gradient descent program over the following types, in
which the dimensions are run-time data:

| Type | Representation |
|---|---|
| `Vec` | `std::vec::Vec<f64>`, length known only at run time |
| `Mat` | `std::vec::Vec<std::vec::Vec<f64>>`, rows and cols known only at run time |
| `Linear` | weights `Mat` (out x in) + bias `Vec` (out) |
| `Network` | two `Linear` layers |

Operations whose inputs must agree in dimension return `Option` and signal a
mismatch with `None`. Run the program and observe the output.

The test suite for the final design already lives in `src/tests.rs`, but the
test module is not enabled yet: the `mod tests;` declaration at the bottom of
`src/main.rs` is commented out. You will enable it in Exercise 3.

## Exercise 1: Refactor - Identify the Failing States

1. Introduce predicates $vec(a, n)$ ("$a$ is a vector of dimension $n$") and
   $matrix(a, m, n)$ ("$a$ is an $m \times n$ matrix"). State the dimension
   condition under which each of the following is well-formed: adding two
   vectors, the dot product of two vectors, multiplying a matrix by a
   vector, and multiplying two matrices. TODOs: fill in the ... below

   * adding two vectors:
   $\forall a.\ \forall b.\ \forall m.\ \forall n.\
   vec(a,m) \wedge vec(b,n) \wedge m = n \implies ... $
   * dot product of two vectors:
   $\forall a.\ \forall b.\ \forall m.\ \forall n.\
   vec(a,m) \wedge vec(b,n) \wedge m = n \implies dot(a,b) \in \mathbb{R}$
   (the result is a scalar, so it carries no dimension)
   * matrix by vector:
   $\forall a.\ \forall v.\ \forall m.\ \forall n.\
   matrix(a,m,n) \wedge vec(v,n) \implies ... $
   * matrix by matrix:
   $\forall a.\ \forall b.\ \forall x.\ \forall y.\ \forall z.\
   matrix(a,x,y) \wedge matrix(b,y,z) \implies ... $

2. In the output of `main`, identify the values that are `None`, and the
   calls guarded by `expect`. For each, state which dimension condition was
   being checked at run time. Hints, list all the functions that returns an Option type and 
   their call sites.

3. Why is a run-time `None` a weaker guarantee than a compile-time error?
   Consider who has to handle it, what happens if a caller forgets, and what
   the type system tells the caller about the dimensions.

4. Complete the table for every operation in `src/main.rs` that returns
   `Option`: state the exact dimension condition it checks.

| Operation | Condition checked at run time |
|---|---|
| `Vec::add` / `Vec::sub` / `Vec::mul_elem` | |
| `Vec::dot` | |
| `Mat::mul` | |
| `Mat::mul_vec` | |
| `Mat::add` | |
| `Linear::new` | |
| `Linear::forward` / `mse_loss` / `grad_weights` / `grad_bias` | |
| `Network::new` | |

## Exercise 2: Spec - Derive the Refined Types (Iteration 2)

We want to make the failing states untypeable: an operation should only be
callable when the dimensions already agree, and the dimensions of the result
should be forced by the dimensions of the arguments.

The notes established the core predicate for matrix multiplication:

$$
\forall x.\ \forall y.\ \forall z.\ \forall a.\ \forall b.\
matrix(a,x,y) \wedge matrix(b,y,z) \implies matrix(matmul(a,b),x,z)
$$

Following the constant-generic pattern from the notes, answer the following.

1. Define the refined types `Vec`, `Mat`, `Linear`, and `Network` with their
   const parameters, and state what each const parameter denotes. Also state
   the types of the fields of `Linear` and `Network`.

   (Reference: `Vec<const N>`, `Mat<const R, const C>`,
   `Linear<const IN, const OUT>`, `Network<const IN, const H, const OUT>`.)

2. For each operation below, write the refactored type signature (receiver
   type, argument types, return type) and justify it with a logical statement
   of the form $\dots \implies \dots$, or explain why no condition is needed.

| Operation | Refactored signature | Justification |
|---|---|---|
| `Vec::add` | | |
| `Vec::dot` | | |
| `Mat::transpose` | | |
| `Mat::mul` | | |
| `Mat::mul_vec` | | |
| `Mat::add` | | |
| `Linear::forward` | | |
| `Linear::grad_weights` | | |
| `Linear::grad_bias` | | |
| `Linear::step` | | |
| `Network::new` | | |
| `Network::forward` | | |

3. Explain, in terms of the types, why `Network<IN, H, OUT>` can only be
   constructed from two layers whose hidden dimension agrees, and what the
   run-time check inside `Network::new` in iteration 1 becomes.

After deriving the signatures, check them against `scaffold/main.rs`.

## Exercise 3: Code - Implement the Refined Type (Iteration 2)

Replace `src/main.rs` with `scaffold/main.rs`. It is the outcome of the Spec
phase: the types and signatures are the specification, the bodies are
`todo!()`. The `main` function is the agreed end-to-end interface; do not
change it.

Implement the following **by hand**; each follows directly from the signature
and the math:

- `Vec`: `new`, `zero`, `add`, `sub`, `scale`, `dot`, `mul_elem`, `print`
- `Mat`: `new`, `zero`, `transpose`, `add`, `print`
- the `Clone` impls for `Vec<N>` and `Mat<R, C>`
- `Linear`: `new`, `forward`, `mse_loss`, `grad_bias`
- `Network`: `new`, `forward`, `mse_loss`

Let an **AI agent** generate the following, using the type signatures as the
specification:

- `Mat::mul` - the `(R x K) * (K x C) = (R x C)` product
- `Mat::mul_vec` - the `(R x C) * Vec<C> = Vec<R>` product
- `Linear::grad_weights` - the outer product giving an `(OUT x IN)` matrix
- `Linear::step` and `Network::step` - the gradient updates and the backprop
  loop

For each, give the agent the signature and the meaning of the dimensions;
verify that the generated code type-checks. In your write-up, paste the
prompts you used and the code the agent generated.

When your implementation type-checks, **uncomment the test module** at the
bottom of `src/main.rs` so that it reads

```rust
#[cfg(test)]
mod tests;
```

and run

```bash
cargo test
```

All the tests already present in `src/tests.rs` must pass. Then `cargo run`
should print the expected output as follows,

```
=== Single-layer: Learning y = 2x + 1 ===
  epoch 0: loss = 6.250000, pred(1.0) = 1.5000, expected = 3.0
  epoch 20: loss = 0.000000, pred(1.0) = 2.9999, expected = 3.0
  epoch 40: loss = 0.000000, pred(1.0) = 3.0000, expected = 3.0
  epoch 60: loss = 0.000000, pred(1.0) = 3.0000, expected = 3.0
  epoch 80: loss = 0.000000, pred(1.0) = 3.0000, expected = 3.0
  epoch 99: loss = 0.000000, pred(1.0) = 3.0000, expected = 3.0

=== Two-layer network: 3 -> 4 -> 2 ===
  epoch 0: loss = 0.947250, pred = [-0.2591, 0.4578]
  epoch 10: loss = 0.589434, pred = [-0.0231, 0.2914]
  epoch 20: loss = 0.406698, pred = [0.1330, 0.1871]
  epoch 30: loss = 0.298216, pred = [0.2477, 0.1170]
  epoch 40: loss = 0.227112, pred = [0.3381, 0.0678]
  epoch 49: loss = 0.181538, pred = [0.4057, 0.0358]

  Network dimensions (3->4->2) verified at compile time.
```


## Exercise 4: Refactor - The Illegal States are Untypeable

1. In your refactored program, show the compiler error for each of the
   dimension mismatches listed (commented out) in `main`: a `Vec<2>` added to
   a `Vec<3>`, a `Mat<2, 3>` multiplied by a `Vec<2>`, and a `Mat<2, 3>`
   multiplied by a `Mat<2, 2>`. State which predicate each error enforces.
2. In iteration 1, the same mismatches were detected by returning `None`.
   What did every caller have to do, and what can go wrong if a caller
   forgets? Relate your answer to the `expect` calls in the starting `main`.
3. `Mat::mul` has a const parameter `K` that does not name the receiver.
   Explain where `K` is inferred from, and why the signature
   `fn mul<const K: usize>(&self, other: &Mat<C, K>) -> Mat<R, K>` encodes
   the inner-dimension condition of the predicate.
4. (Optional) Why does `Linear<IN, OUT>` store its weights as `Mat<OUT, IN>`
   rather than `Mat<IN, OUT>`? What would `forward` have to do if the
   weights were stored the other way round?
