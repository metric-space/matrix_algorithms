use rand::prelude::*;
use num_traits::zero;


#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    rows: usize,
    cols: usize,
    vec: Vec<Vec<f32>>
}


impl Matrix{
    fn multiply(&self, x:Matrix) -> Matrix {
        assert!(self.cols == x.rows);

        let mut result: Vec<Vec<f32>> = Vec::new();

        for r in 0..self.rows {
            let mut temp = Vec::new();
            for z in 0..x.cols {
                let mut sum = zero::<f32>();
                for c in 0..self.cols {
                   sum += self.vec[r][c] * x.vec[c][z];
                }
                temp.push(sum);
            }
            result.push(temp);
        }
        Matrix {
            rows: self.rows,
            cols: x.cols,
            vec:result
        }
    }
}


// I need to bring in a vector
fn make_random_vector (r : & mut rand::rngs::ThreadRng , size: usize) -> Matrix {
    let mut result = Vec::new();
    for _ in 0..size {
        let t = r.gen::<f32>();
        result.push(vec![t]);
    }
    Matrix {rows:2, cols:1, vec:result}
}


fn norm(input_vector: Matrix) -> f32 {
    assert!(input_vector.cols == 1);
    f32::powf(
        input_vector.vec.into_iter().flatten().map(|x| x*x ).sum::<f32>(),
        0.5)
}


fn normalize_vector(input_vector: Matrix) -> Matrix {
    let n = norm(input_vector.clone());
    let a = input_vector
        .vec
        .into_iter()
        .map(|x| x.into_iter().map(|y| y/n).collect())
        .collect();
    Matrix {rows:2, cols:1, vec: a}
}

fn power(r: & mut ThreadRng, a:Matrix, iterations: usize) -> Matrix {
    let mut vector = make_random_vector(r, a.vec.len());

    for _ in 0..iterations {
        vector = normalize_vector(a.multiply(vector));
    }
    vector
}



fn main() {

    let mut rng = rand::thread_rng();

    let a = Matrix {rows:2, cols:2 , vec:vec![vec![0.5,0.5], vec![0.2,0.8]]};
    let Matrix{vec: biggest_eigenvector, ..} = power(& mut rng, a, 10);
    println!("{:?}", biggest_eigenvector);
}



// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test(){
        let c = Matrix {rows:2,cols:1, vec:vec![vec![3.0], vec![4.0]]};
        let d = normalize_vector(c);
        assert!(d == Matrix {rows:2,cols:1, vec:vec![vec![3.0/5.0], vec![4.0/5.0]]});
    }
}
