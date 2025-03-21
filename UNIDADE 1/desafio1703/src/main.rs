use desafio1703::multiply_array;

fn main() {
    let arr = [2, 3, 4];
    let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
    println!("The product of the array is: {}", product);
}