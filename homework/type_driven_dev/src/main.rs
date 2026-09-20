// Iteration 1: dimensions are run-time data.
// Operations whose inputs must agree in dimension return `Option` and
// signal a mismatch with `None`.

// Note: our `Vec` shadows the standard library's `Vec`, so the standard
// one is written with its full path, `std::vec::Vec`.

#[derive(Debug)]
struct Vec {
    data: std::vec::Vec<f64>,
}

impl Vec {
    fn new(data: std::vec::Vec<f64>) -> Self {
        Vec { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)]
    fn zero(n: usize) -> Self {
        Vec { data: vec![0.0; n] }
    }

    // Element-wise addition: requires equal lengths.
    fn add(&self, other: &Vec) -> Option<Vec> {
        if self.data.len() != other.data.len() {
            return None;
        }
        Some(Vec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        })
    }

    // Element-wise subtraction: requires equal lengths.
    fn sub(&self, other: &Vec) -> Option<Vec> {
        if self.data.len() != other.data.len() {
            return None;
        }
        Some(Vec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect(),
        })
    }

    // Scalar multiplication: no dimension constraint.
    fn scale(&self, scalar: f64) -> Vec {
        Vec {
            data: self.data.iter().map(|a| a * scalar).collect(),
        }
    }

    // Dot product: requires equal lengths.
    fn dot(&self, other: &Vec) -> Option<f64> {
        if self.data.len() != other.data.len() {
            return None;
        }
        Some(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .sum(),
        )
    }

    // Element-wise multiplication: requires equal lengths.
    #[allow(dead_code)]
    fn mul_elem(&self, other: &Vec) -> Option<Vec> {
        if self.data.len() != other.data.len() {
            return None;
        }
        Some(Vec {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .collect(),
        })
    }

    #[allow(dead_code)]
    fn print(&self, label: &str) {
        println!("  {} (dim {}): {:?}", label, self.data.len(), self.data);
    }
}

#[derive(Debug)]
struct Mat {
    data: std::vec::Vec<std::vec::Vec<f64>>,
}

impl Mat {
    fn new(data: std::vec::Vec<std::vec::Vec<f64>>) -> Self {
        Mat { data }
    }

    fn zero(rows: usize, cols: usize) -> Self {
        Mat {
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }

    // Transpose: swaps rows and cols.
    fn transpose(&self) -> Mat {
        if self.data.is_empty() {
            return Mat { data: vec![] };
        }
        let cols = self.data[0].len();
        Mat {
            data: (0..cols)
                .map(|j| self.data.iter().map(|row| row[j]).collect())
                .collect(),
        }
    }

    // Matrix multiplication: requires self.cols == other.rows.
    fn mul(&self, other: &Mat) -> Option<Mat> {
        if self.cols() != other.rows() {
            return None;
        }
        let mut data = std::vec::Vec::with_capacity(self.rows());
        for i in 0..self.rows() {
            let mut row = std::vec::Vec::with_capacity(other.cols());
            for j in 0..other.cols() {
                let mut sum = 0.0;
                for k in 0..self.cols() {
                    sum += self.data[i][k] * other.data[k][j];
                }
                row.push(sum);
            }
            data.push(row);
        }
        Some(Mat { data })
    }

    // Matrix-vector multiplication: requires self.cols == v.len().
    fn mul_vec(&self, v: &Vec) -> Option<Vec> {
        if self.cols() != v.len() {
            return None;
        }
        let mut data = std::vec::Vec::with_capacity(self.rows());
        for i in 0..self.rows() {
            let mut sum = 0.0;
            for j in 0..self.cols() {
                sum += self.data[i][j] * v.data[j];
            }
            data.push(sum);
        }
        Some(Vec { data })
    }

    // Element-wise addition: requires equal dimensions.
    #[allow(dead_code)]
    fn add(&self, other: &Mat) -> Option<Mat> {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return None;
        }
        Some(Mat {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(r1, r2)| {
                    r1.iter()
                        .zip(r2.iter())
                        .map(|(a, b)| a + b)
                        .collect()
                })
                .collect(),
        })
    }

    #[allow(dead_code)]
    fn print(&self, label: &str) {
        println!("  {} ({}x{}):", label, self.rows(), self.cols());
        for row in &self.data {
            println!("    {:?}", row);
        }
    }
}

// Linear model: y = W . x + b
// W is (out x in), x has length in, b has length out.
struct Linear {
    weights: Mat,
    bias: Vec,
}

impl Linear {
    fn new(weights: Mat, bias: Vec) -> Option<Linear> {
        if weights.rows() != bias.len() {
            return None;
        }
        Some(Linear { weights, bias })
    }

    fn input_dim(&self) -> usize {
        self.weights.cols()
    }

    fn output_dim(&self) -> usize {
        self.weights.rows()
    }

    // Forward pass: W @ x + b
    fn forward(&self, x: &Vec) -> Option<Vec> {
        if x.len() != self.input_dim() {
            return None;
        }
        let wx = self.weights.mul_vec(x)?;
        wx.add(&self.bias)
    }

    // Compute MSE loss: mean((y_pred - y_true)^2)
    fn mse_loss(&self, x: &Vec, y_true: &Vec) -> Option<f64> {
        if y_true.len() != self.output_dim() {
            return None;
        }
        let y_pred = self.forward(x)?;
        let diff = y_pred.sub(y_true)?;
        diff.dot(&diff).map(|sq| sq / self.output_dim() as f64)
    }

    // Compute gradient w.r.t. weights: (2/n) * (y_pred - y_true) * x^T
    fn grad_weights(&self, x: &Vec, y_true: &Vec) -> Option<Mat> {
        let y_pred = self.forward(x)?;
        let error = y_pred.sub(y_true)?.scale(2.0 / self.output_dim() as f64);

        // outer product: error(out) * x^T(in) -> out x in
        let mut data = std::vec::Vec::with_capacity(error.len());
        for i in 0..error.len() {
            data.push((0..x.len()).map(|j| error.data[i] * x.data[j]).collect());
        }
        Some(Mat { data })
    }

    // Compute gradient w.r.t. bias: (2/n) * (y_pred - y_true)
    fn grad_bias(&self, x: &Vec, y_true: &Vec) -> Option<Vec> {
        let y_pred = self.forward(x)?;
        y_pred
            .sub(y_true)
            .map(|e| e.scale(2.0 / self.output_dim() as f64))
    }

    // Gradient descent step: param = param - lr * grad
    fn step(&mut self, lr: f64, x: &Vec, y_true: &Vec) -> bool {
        let gw = match self.grad_weights(x, y_true) {
            Some(g) => g,
            None => return false,
        };
        let gb = match self.grad_bias(x, y_true) {
            Some(g) => g,
            None => return false,
        };
        update_layer(self, &gw, &gb, lr);
        true
    }
}

// Multi-layer network: layer1 -> layer2
struct Network {
    layer1: Linear,
    layer2: Linear,
}

impl Network {
    fn new(layer1: Linear, layer2: Linear) -> Option<Network> {
        if layer1.output_dim() != layer2.input_dim() {
            return None;
        }
        Some(Network { layer1, layer2 })
    }

    // Forward: x -> layer1 -> layer2 -> output
    fn forward(&self, x: &Vec) -> Option<Vec> {
        let h = self.layer1.forward(x)?;
        self.layer2.forward(&h)
    }

    fn mse_loss(&self, x: &Vec, y_true: &Vec) -> Option<f64> {
        let y_pred = self.forward(x)?;
        let diff = y_pred.sub(y_true)?;
        diff.dot(&diff).map(|sq| sq / self.layer2.output_dim() as f64)
    }

    // Simplified backprop for demonstration
    fn step(&mut self, lr: f64, x: &Vec, y_true: &Vec) -> bool {
        let h = match self.layer1.forward(x) {
            Some(h) => h,
            None => return false,
        };

        // Gradient for layer2
        let g2w = match self.layer2.grad_weights(&h, y_true) {
            Some(g) => g,
            None => return false,
        };
        let g2b = match self.layer2.grad_bias(&h, y_true) {
            Some(g) => g,
            None => return false,
        };
        update_layer(&mut self.layer2, &g2w, &g2b, lr);

        // Gradient signal back to layer1
        let g2w_t = g2w.transpose();
        let d_h = match g2w_t.mul_vec(y_true) {
            Some(d) => d,
            None => return false,
        };
        let g1w = match self.layer1.grad_weights(x, &d_h) {
            Some(g) => g,
            None => return false,
        };
        let g1b = match self.layer1.grad_bias(x, &d_h) {
            Some(g) => g,
            None => return false,
        };
        update_layer(&mut self.layer1, &g1w, &g1b, lr);
        true
    }
}

// param = param - lr * grad, in place
fn update_layer(layer: &mut Linear, gw: &Mat, gb: &Vec, lr: f64) {
    for i in 0..layer.weights.rows() {
        for j in 0..layer.weights.cols() {
            layer.weights.data[i][j] -= lr * gw.data[i][j];
        }
    }
    for i in 0..layer.bias.len() {
        layer.bias.data[i] -= lr * gb.data[i];
    }
}

fn main() {
    // === Single-layer gradient descent ===
    // Learning y = 2x + 1 with a Linear model
    println!("=== Single-layer: Learning y = 2x + 1 ===");

    let mut model = Linear::new(Mat::new(vec![vec![0.5]]), Vec::new(vec![0.0]))
        .expect("Linear::new dimension mismatch");

    let lr = 0.1;
    for epoch in 0..100 {
        let x = Vec::new(vec![1.0]);
        let y = Vec::new(vec![3.0]); // y = 2*1 + 1 = 3
        let loss = model
            .mse_loss(&x, &y)
            .expect("mse_loss dimension mismatch");
        model.step(lr, &x, &y);

        if epoch % 20 == 0 || epoch == 99 {
            let pred = model.forward(&x).expect("forward dimension mismatch");
            println!(
                "  epoch {}: loss = {:.6}, pred(1.0) = {:.4}, expected = 3.0",
                epoch, loss, pred.data[0]
            );
        }
    }

    // === Dimension mismatches are detected at run time ===
    // Nothing in the type system prevents these calls; the operations
    // simply return None.
    println!("\n=== Dimension mismatch (run time) ===");
    let a = Vec::new(vec![1.0, 2.0]);
    let b = Vec::new(vec![1.0, 2.0, 3.0]);
    println!("  a.add(&b)    = {:?}", a.add(&b));
    println!("  a.dot(&b)    = {:?}", a.dot(&b));

    let m = Mat::zero(2, 3);
    let v = Vec::new(vec![1.0, 2.0]);
    println!("  m.mul_vec(&v) = {:?}", m.mul_vec(&v));

    let m1 = Mat::zero(2, 3);
    let m2 = Mat::zero(2, 2);
    println!("  m1.mul(&m2)  = {:?}", m1.mul(&m2));

    // === Two-layer network: 3 -> 4 -> 2 ===
    println!("\n=== Two-layer network: 3 -> 4 -> 2 ===");

    let net = Network::new(
        Linear::new(
            Mat::new(vec![
                vec![0.1, -0.2, 0.3],
                vec![0.4, 0.5, -0.1],
                vec![-0.3, 0.2, 0.1],
                vec![0.2, 0.1, -0.4],
            ]),
            Vec::new(vec![0.0, 0.0, 0.0, 0.0]),
        )
        .expect("layer1 dimension mismatch"),
        Linear::new(
            Mat::new(vec![
                vec![0.5, -0.3, 0.2, 0.1],
                vec![-0.1, 0.4, -0.2, 0.3],
            ]),
            Vec::new(vec![0.0, 0.0]),
        )
        .expect("layer2 dimension mismatch"),
    )
    .expect("Network::new dimension mismatch");

    let x = Vec::new(vec![1.0, 0.5, -0.5]);
    let y = Vec::new(vec![1.0, 0.0]);
    let mut net = net;

    for epoch in 0..50 {
        let loss = net.mse_loss(&x, &y).expect("mse_loss dimension mismatch");
        net.step(0.01, &x, &y);

        if epoch % 10 == 0 || epoch == 49 {
            let pred = net.forward(&x).expect("forward dimension mismatch");
            println!(
                "  epoch {}: loss = {:.6}, pred = [{:.4}, {:.4}]",
                epoch, loss, pred.data[0], pred.data[1]
            );
        }
    }

    // The dimensions (3 -> 4 -> 2) were checked at run time,
    // not at compile time.
    println!("\n  Network dimensions (3->4->2) checked at run time.");
}

// The test suite for the final (iteration 2) design lives in `tests.rs`.
// It is not enabled yet: uncomment the following lines once your
// refactored program is complete (see Exercise 3).
//
// #[cfg(test)]
// mod tests;
