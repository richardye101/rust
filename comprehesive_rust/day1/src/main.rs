// Exercise 6.7 - Control Flow: Collatz sequence
fn collatz_length(mut n: i32) -> u32 {
    let mut count = 1;
    while n > 1 {
        n = if n % 2 == 0 {
            n/2
        } else {
            3 * n + 1
        };
        count+=1;
    }
    count
}

// Exercise 8.5 - Tuples and Arrays: Nested Arrays
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed: [[i32; 3]; 3] = matrix; 
    for row in 0..3{
        for col in 0..3{
            transposed[col][row] = matrix[row][col];
        }
    }
    transposed
} 

// Exercise 9.3 - References: Geometry

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag = 0.0;
    for i in vector {
        mag += i * i;
    }
    mag.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(vector: &mut[f64; 3]) {
    let mag = magnitude(vector);
    for i in vector{
        *i /= mag;
    }
}

fn main() {
    println!("Exercise 6.7 - Control Flow: {}", collatz_length(256));


    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];  
    println!("Exercise 8.5 - Nested Arrays: original {:?} \n transposed {:#?}", matrix, transpose(matrix));


    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
