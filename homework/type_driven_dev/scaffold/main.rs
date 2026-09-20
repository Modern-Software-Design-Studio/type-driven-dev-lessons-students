// Iteration 2 (Spec phase outcome): dimensions are compile-time constants.
// The types and signatures are the specification; the bodies are `todo!()`.
// The `main` function is the agreed end-to-end interface; do not change it.

// A vector with compile-time known dimension
struct Vec<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Vec<N> {
    fn new(data: [f64; N]) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn zero() -> Self {
        todo!()
    }

    // Element-wise addition: N + N = N
    fn add(self, other: Self) -> Self {
        todo!()
    }

    // Element-wise subtraction: N - N = N
    fn sub(self, other: Self) -> Self {
        todo!()
    }

    // Scalar multiplication: scalar * N = N
    fn scale(self, scalar: f64) -> Self {
        todo!()
    }

    // Dot product: N . N = scalar
    fn dot(&self, other: &Self) -> f64 {
        todo!()
    }

    // Element-wise multiplication: N .* N = N
    #[allow(dead_code)]
    fn mul_elem(&self, other: &Self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn print(&self, label: &str) {
        todo!()
    }
}

// A matrix with compile-time known dimensions
struct Mat<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Mat<R, C> {
    fn new(data: [[f64; C]; R]) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn zero() -> Self {
        todo!()
    }

    // Transpose: R x C -> C x R
    fn transpose(&self) -> Mat<C, R> {
        todo!()
    }

    // Matrix multiplication: (R x K) * (K x C) = (R x C)
    #[allow(dead_code)]
    fn mul<const K: usize>(&self, other: &Mat<C, K>) -> Mat<R, K> {
        todo!()
    }

    // Matrix-vector multiplication: (R x C) * Vec<C> = Vec<R>
    fn mul_vec(&self, v: &Vec<C>) -> Vec<R> {
        todo!()
    }

    // Element-wise addition: R x C + R x C = R x C
    fn add(self, other: Self) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn print(&self, label: &str) {
        todo!()
    }
}

// Linear model: y = W . x + b
// W: (out x in), x: Vec<in>, b: Vec<out>, result: Vec<out>
struct Linear<const IN: usize, const OUT: usize> {
    weights: Mat<OUT, IN>,
    bias: Vec<OUT>,
}

impl<const IN: usize, const OUT: usize> Linear<IN, OUT> {
    fn new(weights: Mat<OUT, IN>, bias: Vec<OUT>) -> Self {
        todo!()
    }

    // Forward pass: W @ x + b
    fn forward(&self, x: &Vec<IN>) -> Vec<OUT> {
        todo!()
    }

    // Compute MSE loss: mean((y_pred - y_true)^2)
    fn mse_loss(&self, x: &Vec<IN>, y_true: &Vec<OUT>) -> f64 {
        todo!()
    }

    // Compute gradient w.r.t. weights: (2/n) * (y_pred - y_true) . x^T
    // Returns gradient of shape (OUT x IN)
    fn grad_weights(&self, x: &Vec<IN>, y_true: &Vec<OUT>) -> Mat<OUT, IN> {
        todo!()
    }

    // Compute gradient w.r.t. bias: (2/n) * (y_pred - y_true)
    fn grad_bias(&self, x: &Vec<IN>, y_true: &Vec<OUT>) -> Vec<OUT> {
        todo!()
    }

    // Gradient descent step: param = param - lr * grad
    fn step(&mut self, lr: f64, x: &Vec<IN>, y_true: &Vec<OUT>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> Clone for Mat<R, C> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<const N: usize> Clone for Vec<N> {
    fn clone(&self) -> Self {
        todo!()
    }
}

// Multi-layer network: Linear<IN, H> -> Linear<H, OUT>
struct Network<const IN: usize, const H: usize, const OUT: usize> {
    layer1: Linear<IN, H>,
    layer2: Linear<H, OUT>,
}

impl<const IN: usize, const H: usize, const OUT: usize> Network<IN, H, OUT> {
    fn new(layer1: Linear<IN, H>, layer2: Linear<H, OUT>) -> Self {
        todo!()
    }

    // Forward: x -> layer1 -> layer2 -> output
    fn forward(&self, x: &Vec<IN>) -> Vec<OUT> {
        todo!()
    }

    fn mse_loss(&self, x: &Vec<IN>, y_true: &Vec<OUT>) -> f64 {
        todo!()
    }

    // Simplified backprop for demonstration
    fn step(&mut self, lr: f64, x: &Vec<IN>, y_true: &Vec<OUT>) {
        todo!()
    }
}

fn main() {
    // === Single-layer gradient descent ===
    // Learning y = 2x + 1 with a Linear<1, 1> model
    println!("=== Single-layer: Learning y = 2x + 1 ===");

    let mut model = Linear::<1, 1>::new(
        Mat::<1, 1>::new([[0.5]]),
        Vec::<1>::new([0.0]),
    );

    let lr = 0.1;
    for epoch in 0..100 {
        let x = Vec::<1>::new([1.0]);
        let y = Vec::<1>::new([3.0]); // y = 2*1 + 1 = 3
        let loss = model.mse_loss(&x, &y);
        model.step(lr, &x, &y);

        if epoch % 20 == 0 || epoch == 99 {
            let pred = model.forward(&x);
            println!(
                "  epoch {}: loss = {:.6}, pred(1.0) = {:.4}, expected = 3.0",
                epoch, loss, pred.data[0]
            );
        }
    }

    // === Dimension mismatch caught at compile time ===
    // Uncommenting any of these would fail to compile:
    //
    // let a = Vec::<2>::new([1.0, 2.0]);
    // let b = Vec::<3>::new([1.0, 2.0, 3.0]);
    // a.add(b);       // error: dimension mismatch (2 vs 3)
    // a.dot(&b);      // error: dimension mismatch (2 vs 3)
    //
    // let m = Mat::<2, 3>::zero();
    // let v = Vec::<2>::new([1.0, 2.0]);
    // m.mul_vec(&v);  // error: matrix is 2x3 but vec is dim 2 (needs dim 3)
    //
    // let m1 = Mat::<2, 3>::zero();
    // let m2 = Mat::<2, 2>::zero();
    // m1.mul(&m2);    // error: inner dims don't match (3 vs 2)

    // === Two-layer network: 3 -> 4 -> 2 ===
    println!("\n=== Two-layer network: 3 -> 4 -> 2 ===");

    let net = Network::<3, 4, 2>::new(
        Linear::new(
            Mat::<4, 3>::new([
                [0.1, -0.2, 0.3],
                [0.4, 0.5, -0.1],
                [-0.3, 0.2, 0.1],
                [0.2, 0.1, -0.4],
            ]),
            Vec::<4>::new([0.0, 0.0, 0.0, 0.0]),
        ),
        Linear::new(
            Mat::<2, 4>::new([
                [0.5, -0.3, 0.2, 0.1],
                [-0.1, 0.4, -0.2, 0.3],
            ]),
            Vec::<2>::new([0.0, 0.0]),
        ),
    );

    let x = Vec::<3>::new([1.0, 0.5, -0.5]);
    let y = Vec::<2>::new([1.0, 0.0]);
    let mut net = net;

    for epoch in 0..50 {
        let loss = net.mse_loss(&x, &y);
        net.step(0.01, &x, &y);

        if epoch % 10 == 0 || epoch == 49 {
            let pred = net.forward(&x);
            println!(
                "  epoch {}: loss = {:.6}, pred = [{:.4}, {:.4}]",
                epoch, loss, pred.data[0], pred.data[1]
            );
        }
    }

    // Compile-time guarantee: Network<3,4,2> enforces
    // layer1: Linear<3,4> -> layer2: Linear<4,2>
    // The hidden dimension H=4 must match between layers.
    // This is checked at compile time, not runtime.
    println!("\n  Network dimensions (3->4->2) verified at compile time.");
}

// The test suite for this design lives in `tests.rs`. Uncomment the
// following lines once your implementation type-checks (see Exercise 3).
//
// #[cfg(test)]
// mod tests;
