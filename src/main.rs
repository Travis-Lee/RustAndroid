mod matrix;

fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6]
    ];
    
    let matrix_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12]
    ];
    
    match matrix::multiply_matrices(&matrix_a, &matrix_b) {
        Ok(result) => {
            println!("Matrix A:");
            matrix::print_matrix(&matrix_a);
            
            println!("\nMatrix B:");
            matrix::print_matrix(&matrix_b);
            
            println!("\nMulti Matrix Result (A × B):");
            matrix::print_matrix(&result);
        },
        Err(e) => println!("ERROR: {}", e)
    }
}


