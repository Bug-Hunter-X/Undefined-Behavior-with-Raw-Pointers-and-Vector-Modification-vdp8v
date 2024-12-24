fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!( "First element: {:?}", v[0]); // Output: First element: 4
    v.push(4); // This line will cause undefined behavior
    println!("Vector: {:?}", v); // Output: may be unpredictable
}