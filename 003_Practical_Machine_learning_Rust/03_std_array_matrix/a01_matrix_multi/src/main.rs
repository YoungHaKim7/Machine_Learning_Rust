fn matrix_multiply(matrix: [[i32; 3]; 3], mul: [[i32; 1]; 3]) -> [[i32; 1]; 3] {
    let mut result = [[0; 1]; 3];

    for i in 0..3 {
        for j in 0..1 {
            for k in 0..3 {
                result[i][j] += matrix[i][k] * mul[k][j];
            }
        }
    }

    result
}

fn main() {
    let matrix_2d = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mat_mul = [[1], [2], [3]];

    let mat_result = matrix_multiply(matrix_2d, mat_mul);

    println!("Matrix 2D:");
    for row in matrix_2d {
        println!("{:?}", row);
    }

    println!("\nMatrix Mul:");
    for row in mat_mul {
        println!("{:?}", row);
    }

    println!("\nResult:");
    for row in mat_result {
        println!("{:?}", row);
    }
}
