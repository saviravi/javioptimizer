use ndarray::Array2;

fn init_simplex(A: Array2<f64>, b: Vec<f64>, c: Vec<f64>) -> Option<(Array2<usize>, Vec<usize>, Array2<f64>, Vec<f64>, Vec<f64>, i32)> {
    // check if A is m x n
    let (m, n) = A.dim();
    if m > n {
        panic!("Number of rows in A must be less than or equal to the number of columns");
    }
    // check if b is m x 1
    if b.len() != m {
        panic!("Number of rows in b must be equal to the number of rows in A");
    }
    // check if c is 1 x n
    if c.len() != n {
        panic!("Number of columns in c must be equal to the number of columns in A");
    }
    else {
        // return N B A b c v
        return Some((Array2::<usize>::zeros((0, 0)), (0..n).collect(), A.clone(), b, c, 1));
    }

}

fn simplex(A: Array2<f64>, b: Vec<f64>, c: Vec<f64>) -> Result<Vec<usize>, ()> {
    let (mut nonbasic_vars, mut basic_vars, A, b, c, v) = init_simplex(A, b, c).unwrap();
    let (m, n) = A.dim();
    let mut delta: Vec<f64> = vec![0.0; m];
    let mut xs: Vec<usize> = vec![0; n];

    loop {
        // choose an index e in nonbasic_vars s.t. c[e] > 0
        if let Some(e) = nonbasic_vars.iter().find(|&&i| c[i] > 0.0) {
            for idx in &b {
                let i = *idx as usize;
                // if A[i][e] > 0 set delta[i] = b[i] / A[i][e]
                if A[[i, *e]] > 0.0 {
                    delta[i] = b[i] / A[[i, *e]];
                } else {
                    delta[i] = std::f64::INFINITY;
                }
            }
            // choose the smallest index l in B that minimizes delta[l]
            
            let (min_idx, min_val) = delta.iter().enumerate().fold((0, std::f64::INFINITY), |acc, (i, &val)| {
                if val < acc.1 {
                    (i, val)
                } else {
                    acc
                }
            });
            
            if min_val == std::f64::INFINITY {
                return Err(());
            } else {
                (nonbasic_vars, basic_vars, _, _, _, _) = pivot(nonbasic_vars, basic_vars, A.clone(), &b, &c, v, min_idx as i32, 0);
            }

            for i in 0..n {
                let i_idx = basic_vars.iter().position(|&x| x == i);
                match i_idx {
                    Some(_) => {
                        // if i is in B
                        xs[i] = basic_vars[i];
                    }
                    None => {
                        xs[i] = 0;
                    }
                }
            }
        } else {
            break;
        }
    }
    Ok(xs)
}

fn pivot(nonbasic_vars: Array2<usize>, basic_vars: Vec<usize>, A: Array2<f64>, b: &Vec<f64>, c: &Vec<f64>, v: i32, leaving_idx: i32, entering_idx: i32) -> (Array2<usize>, Vec<usize>, Array2<f64>, Vec<f64>, Vec<f64>, i32) {
    return (nonbasic_vars, basic_vars, A, b.to_vec(), c.to_vec(), v);
}
