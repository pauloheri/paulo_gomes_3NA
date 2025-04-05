// src/lib.rs

pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len {
        unsafe {
            product *= *ptr.offset(i as isize);
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        unsafe {
            let product = multiply_array(arr.as_ptr(), arr.len());
            assert_eq!(product, 24);
        }
    }

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        unsafe {
            let product = multiply_array(arr.as_ptr(), arr.len());
            assert_eq!(product, 1);
        }
    }
}