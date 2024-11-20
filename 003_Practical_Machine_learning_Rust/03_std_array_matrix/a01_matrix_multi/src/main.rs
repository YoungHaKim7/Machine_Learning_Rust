fn matrix_multiply(matrix: [[i32; 3]; 3], mul: [[i32; 0]; 3]) -> [[i32; 0]; 3] {
    let mut result = [[0; 0]; 3];

    for i in 0..3 {
        for j in 0..3 {
            let one_row =
                matrix[i][j] * mul[i][0] + matrix[0][j] * mul[i][0] + matrix[0][j] * mul[i][0];
            for k in 0..3 {
                result[k][0] += one_row;
            }
        }
    }

    result
}

fn main() {
    let mut matrix_2d = [[0; 3]; 3];
    let mut mat_mul = [[1; 1]; 3];

    matrix_2d[0][0] = 1;
    matrix_2d[0][1] = 2;
    matrix_2d[0][2] = 3;
    matrix_2d[1][0] = 4;
    matrix_2d[1][1] = 5;
    matrix_2d[1][2] = 6;
    matrix_2d[2][0] = 7;
    matrix_2d[2][1] = 8;
    matrix_2d[2][2] = 9;
    mat_mul[1][0] = 2;
    mat_mul[2][0] = 3;

    mat_mul = matrix_multiply(matrix_2d, mat_mul);

    for mat_2d in matrix_2d {
        println!("{mat_2d:?}");
    }
    println!();
    for mul in mat_mul {
        println!("{mul:?}");
    }
    println!();
}
