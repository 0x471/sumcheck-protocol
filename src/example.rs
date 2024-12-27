use ark_bls12_381::Fq;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Polynomial,
};

// TODO:
// 1. Implement polynomial evaluation
// 2. Randomize challenge generation
// 3. Implement recursive verification
// 4. Add degree checks
// 5. Integrate Schwartz-Zippel lemma for identity testing

pub fn run_example() {
    let g = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fq::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fq::from(1), SparseTerm::new(vec![(1, 1)])),
        ],
    );
    
    let g_0 = SparsePolynomial::from_coefficients_vec(
        1,
        vec![
            (Fq::from(8), SparseTerm::new(vec![(0, 3)])),
            (Fq::from(2), SparseTerm::new(vec![(0, 1)])),
            (Fq::from(2), SparseTerm::new(vec![])),
        ],
    );

    let g_0_at_0 = g_0.evaluate(&vec![Fq::from(0)]);
    let g_0_at_1 = g_0.evaluate(&vec![Fq::from(1)]);
    let sum_g_0 = g_0_at_0 + g_0_at_1;

    let expected_sum_g_0 = Fq::from(14);
    if sum_g_0 == expected_sum_g_0 {
        println!("[SUCCESS] g_0(0) + g_0(1) == H: {} == {}", sum_g_0, expected_sum_g_0);
    } else {
        println!(
            "[FAILURE] g_0(0) + g_0(1) != g_0(0) + g_0(1): {} != {} \n  Expected: {} \n  Got: {}",
            sum_g_0, sum_g_0, expected_sum_g_0, sum_g_0
        );
    }

    let r_0 = Fq::from(12);

    let g_1 = DensePolynomial::from_coefficients_vec(vec![Fq::from(6924), Fq::from(2)]);

    let g_1_at_0 = g_1.evaluate(&Fq::from(0));
    let g_1_at_1 = g_1.evaluate(&Fq::from(1));
    let sum_g_1 = g_1_at_0 + g_1_at_1;

    let g_0_at_r_0 = g_0.evaluate(&vec![r_0]);

    if sum_g_1 == g_0_at_r_0 {
        println!("[SUCCESS] g_1(0) + g_1(1) == g_0(r_0): {} == {}", sum_g_1, g_0_at_r_0);
    } else {
        println!(
            "[FAILURE] g_1(0) + g_1(1) != g_0(r_0): {} != {} \n  Expected: {} \n  Got: {}",
            sum_g_1, sum_g_1, g_0_at_r_0, sum_g_1
        );
    }

    let r_1 = Fq::from(5);

    let g_2 = DensePolynomial::from_coefficients_vec(vec![Fq::from(3461), Fq::from(12)]);

    let g_2_at_0 = g_2.evaluate(&Fq::from(0));
    let g_2_at_1 = g_2.evaluate(&Fq::from(1));
    let sum_g_2 = g_2_at_0 + g_2_at_1;

    let g_1_at_r_1 = g_1.evaluate(&r_1);

    if sum_g_2 == g_1_at_r_1 {
        println!("[SUCCESS] g_2(0) + g_2(1) == g_1(r_1): {} == {}", sum_g_2, g_1_at_r_1);
    } else {
        println!(
            "[FAILURE] g_2(0) + g_2(1) != g_1(r_1): {} != {} \n  Expected: {} \n  Got: {}",
            sum_g_2, sum_g_2, g_1_at_r_1, sum_g_2
        );
    }

    let r_2 = Fq::from(2);

    let g_at_r_values = g.evaluate(&vec![r_0, r_1, r_2]);
    let g_2_at_r_2 = g_2.evaluate(&r_2);

    if g_at_r_values == g_2_at_r_2 {
        println!(
            "[SUCCESS] g(r_0, r_1, r_2) == g_2(r_2): {} == {}",
            g_at_r_values, g_2_at_r_2
        );
    } else {
        println!(
            "[FAILURE] g(r_0, r_1, r_2) != g_2(r_2): {} != {}",
            g_at_r_values, g_2_at_r_2
        );
    }
}
