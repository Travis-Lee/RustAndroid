pub fn multiply_matrices(a: &[Vec<i32>], b: &[Vec<i32>]) -> Result<Vec<Vec<i32>>, String> {
    if a[0].len() != b.len() {
        return Err(format!("matrix dimension not matching : A is {}×{} Matrix，B is {}×{} Matrix", 
                          a.len(), a[0].len(), b.len(), b[0].len()));
    }
    
    let mut result = vec![vec![0; b[0].len()]; a.len()];
    
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    Ok(result)
}

pub fn print_matrix(matrix: &[Vec<i32>]) {
    for row in matrix {
        for &element in row {
            print!("{:4}", element);
        }
        println!();
    }
}

