// src/main.rs

use biblioteca::multiply_array;

fn main() {
    let arr = [2, 3, 4];
    unsafe {
        let product = multiply_array(arr.as_ptr(), arr.len());
        println!("Produto do array: {}", product);
    }
}