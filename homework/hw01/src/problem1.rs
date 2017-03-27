pub fn sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;

    for x in slice {
        sum += *x;
    }
    sum
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let mut uniques = HashSet::new();

    for x in vs {
        uniques.insert(*x);
    }

    uniques.into_iter().collect()
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut filtered = Vec::new();

    for x in vs {
        if pred(*x) {
            filtered.push(*x);
        }
    }

    filtered
}

pub type Matrix = Vec<Vec<i32>>;

pub mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert(mat1[0].len == mat2.len);

    let mut result = Matrix::new();


    for i in mat1[0].len {
        result
    }
}
