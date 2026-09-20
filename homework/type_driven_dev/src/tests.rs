use super::*;

const EPS: f64 = 1e-9;

fn assert_close(a: f64, b: f64) {
    assert!((a - b).abs() < EPS, "expected {} ~= {}", a, b);
}

// ---------------- Vec<N> ----------------

#[test]
fn test_vec_new() {
    let v = Vec::<3>::new([1.0, 2.0, 3.0]);
    assert_eq!(v.data, [1.0, 2.0, 3.0]);
}

#[test]
fn test_vec_zero() {
    let v = Vec::<3>::zero();
    assert_eq!(v.data, [0.0, 0.0, 0.0]);
}

#[test]
fn test_vec_add() {
    let a = Vec::<3>::new([1.0, 2.0, 3.0]);
    let b = Vec::<3>::new([4.0, 5.0, 6.0]);
    let c = a.add(b);
    assert_eq!(c.data, [5.0, 7.0, 9.0]);
}

#[test]
fn test_vec_sub() {
    let a = Vec::<3>::new([4.0, 5.0, 6.0]);
    let b = Vec::<3>::new([1.0, 2.0, 3.0]);
    let c = a.sub(b);
    assert_eq!(c.data, [3.0, 3.0, 3.0]);
}

#[test]
fn test_vec_scale() {
    let a = Vec::<3>::new([1.0, 2.0, 3.0]);
    let b = a.scale(2.5);
    assert_eq!(b.data, [2.5, 5.0, 7.5]);
}

#[test]
fn test_vec_dot() {
    let a = Vec::<3>::new([1.0, 2.0, 3.0]);
    let b = Vec::<3>::new([4.0, 5.0, 6.0]);
    assert_close(a.dot(&b), 32.0);
}

#[test]
fn test_vec_dot_orthogonal() {
    let a = Vec::<2>::new([1.0, 0.0]);
    let b = Vec::<2>::new([0.0, 1.0]);
    assert_close(a.dot(&b), 0.0);
}

#[test]
fn test_vec_mul_elem() {
    let a = Vec::<3>::new([1.0, 2.0, 3.0]);
    let b = Vec::<3>::new([4.0, 5.0, 6.0]);
    let c = a.mul_elem(&b);
    assert_eq!(c.data, [4.0, 10.0, 18.0]);
}

#[test]
fn test_vec_print_does_not_panic() {
    let v = Vec::<2>::new([1.0, 2.0]);
    v.print("test-vec");
}

#[test]
fn test_vec_clone() {
    let v = Vec::<2>::new([1.0, 2.0]);
    let c = v.clone();
    assert_eq!(c.data, v.data);
}

// ---------------- Mat<R, C> ----------------

#[test]
fn test_mat_new() {
    let m = Mat::<2, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    assert_eq!(m.data, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
}

#[test]
fn test_mat_zero() {
    let m = Mat::<2, 3>::zero();
    assert_eq!(m.data, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
}

#[test]
fn test_mat_transpose() {
    let m = Mat::<2, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let t: Mat<3, 2> = m.transpose();
    assert_eq!(t.data, [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]);
}

#[test]
fn test_mat_transpose_of_transpose() {
    let m = Mat::<2, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let t = m.transpose().transpose();
    assert_eq!(t.data, m.data);
}

#[test]
fn test_mat_mul() {
    let a = Mat::<2, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let b = Mat::<3, 2>::new([[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]]);
    let c: Mat<2, 2> = a.mul(&b);
    assert_eq!(c.data, [[58.0, 64.0], [139.0, 154.0]]);
}

#[test]
fn test_mat_mul_identity() {
    let a = Mat::<2, 2>::new([[1.0, 0.0], [0.0, 1.0]]);
    let b = Mat::<2, 2>::new([[3.0, 4.0], [5.0, 6.0]]);
    let c = a.mul(&b);
    assert_eq!(c.data, b.data);
}

#[test]
fn test_mat_mul_vec() {
    let m = Mat::<2, 3>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    let v = Vec::<3>::new([1.0, 1.0, 1.0]);
    let r = m.mul_vec(&v);
    assert_eq!(r.data, [6.0, 15.0]);
}

#[test]
fn test_mat_add() {
    let a = Mat::<2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
    let b = Mat::<2, 2>::new([[5.0, 6.0], [7.0, 8.0]]);
    let c = a.add(b);
    assert_eq!(c.data, [[6.0, 8.0], [10.0, 12.0]]);
}

#[test]
fn test_mat_print_does_not_panic() {
    let m = Mat::<2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
    m.print("test-mat");
}

#[test]
fn test_mat_clone() {
    let m = Mat::<2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
    let c = m.clone();
    assert_eq!(c.data, m.data);
}

// ---------------- Linear<IN, OUT> ----------------

fn test_linear() -> Linear<2, 2> {
    Linear::new(
        Mat::<2, 2>::new([[1.0, 2.0], [3.0, 4.0]]),
        Vec::<2>::new([1.0, 1.0]),
    )
}

#[test]
fn test_linear_new() {
    let l = test_linear();
    assert_eq!(l.weights.data, [[1.0, 2.0], [3.0, 4.0]]);
    assert_eq!(l.bias.data, [1.0, 1.0]);
}

#[test]
fn test_linear_forward() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = l.forward(&x);
    assert_eq!(y.data, [4.0, 8.0]);
}

#[test]
fn test_linear_forward_zero_input() {
    let l = test_linear();
    let x = Vec::<2>::new([0.0, 0.0]);
    let y = l.forward(&x);
    assert_eq!(y.data, [1.0, 1.0]);
}

#[test]
fn test_linear_mse_loss_exact_fit() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([4.0, 8.0]);
    assert_close(l.mse_loss(&x, &y), 0.0);
}

#[test]
fn test_linear_mse_loss_value() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    assert_close(l.mse_loss(&x, &y), 40.0);
}

#[test]
fn test_linear_grad_weights() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    let g = l.grad_weights(&x, &y);
    assert_eq!(g.data, [[4.0, 4.0], [8.0, 8.0]]);
}

#[test]
fn test_linear_grad_weights_zero_error() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([4.0, 8.0]);
    let g = l.grad_weights(&x, &y);
    assert_eq!(g.data, [[0.0, 0.0], [0.0, 0.0]]);
}

#[test]
fn test_linear_grad_bias() {
    let l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    let g = l.grad_bias(&x, &y);
    assert_eq!(g.data, [4.0, 8.0]);
}

#[test]
fn test_linear_step_updates_parameters() {
    let mut l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    l.step(0.1, &x, &y);
    for i in 0..2 {
        for j in 0..2 {
            assert_close(l.weights.data[i][j], [[0.6, 1.6], [2.2, 3.2]][i][j]);
        }
    }
    assert_close(l.bias.data[0], 0.6);
    assert_close(l.bias.data[1], 0.2);
}

#[test]
fn test_linear_step_reduces_loss() {
    let mut l = test_linear();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    let before = l.mse_loss(&x, &y);
    l.step(0.05, &x, &y);
    let after = l.mse_loss(&x, &y);
    assert!(after < before, "loss should decrease: {} -> {}", before, after);
}

#[test]
fn test_linear_converges_to_target() {
    let mut model = Linear::<1, 1>::new(Mat::<1, 1>::new([[0.5]]), Vec::<1>::new([0.0]));
    let x = Vec::<1>::new([1.0]);
    let y = Vec::<1>::new([3.0]);
    for _ in 0..100 {
        model.step(0.1, &x, &y);
    }
    let pred = model.forward(&x);
    assert_close(pred.data[0], 3.0);
}

// ---------------- Network<IN, H, OUT> ----------------

fn test_network() -> Network<2, 2, 2> {
    Network::new(
        Linear::new(
            Mat::<2, 2>::new([[1.0, 0.0], [0.0, 1.0]]),
            Vec::<2>::new([0.0, 0.0]),
        ),
        Linear::new(
            Mat::<2, 2>::new([[1.0, 0.0], [0.0, 1.0]]),
            Vec::<2>::new([0.0, 0.0]),
        ),
    )
}

#[test]
fn test_network_new() {
    let n = test_network();
    assert_eq!(n.layer1.weights.data, [[1.0, 0.0], [0.0, 1.0]]);
    assert_eq!(n.layer2.weights.data, [[1.0, 0.0], [0.0, 1.0]]);
}

#[test]
fn test_network_forward_identity() {
    let n = test_network();
    let x = Vec::<2>::new([1.0, 2.0]);
    let y = n.forward(&x);
    assert_eq!(y.data, [1.0, 2.0]);
}

#[test]
fn test_network_forward_composed() {
    let n = Network::new(
        Linear::new(
            Mat::<2, 2>::new([[2.0, 0.0], [0.0, 2.0]]),
            Vec::<2>::new([0.0, 0.0]),
        ),
        Linear::new(
            Mat::<2, 2>::new([[3.0, 0.0], [0.0, 3.0]]),
            Vec::<2>::new([0.0, 0.0]),
        ),
    );
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = n.forward(&x);
    assert_eq!(y.data, [6.0, 6.0]);
}

#[test]
fn test_network_mse_loss() {
    let n = test_network();
    let x = Vec::<2>::new([1.0, 2.0]);
    let y = Vec::<2>::new([1.0, 2.0]);
    assert_close(n.mse_loss(&x, &y), 0.0);

    let y2 = Vec::<2>::new([2.0, 4.0]);
    assert_close(n.mse_loss(&x, &y2), 2.5);
}

#[test]
fn test_network_step_reduces_loss() {
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

    let before = net.mse_loss(&x, &y);
    for _ in 0..50 {
        net.step(0.01, &x, &y);
    }
    let after = net.mse_loss(&x, &y);
    assert!(after < before, "loss should decrease: {} -> {}", before, after);
}

#[test]
fn test_network_step_updates_parameters() {
    let mut n = test_network();
    let x = Vec::<2>::new([1.0, 1.0]);
    let y = Vec::<2>::new([0.0, 0.0]);
    let w1_before = n.layer1.weights.clone();
    let w2_before = n.layer2.weights.clone();
    n.step(0.01, &x, &y);
    let changed = n.layer1.weights.data != w1_before.data
        || n.layer2.weights.data != w2_before.data;
    assert!(changed, "a gradient step should change the weights");
}
