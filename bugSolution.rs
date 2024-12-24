fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Modify using safe indexing
    println!("First element: {:?}", v[0]); // Output: First element: 4
    v.push(4); // This is now safe
    println!("Vector: {:?}", v); // Output: Vector: [4, 2, 3, 4]
} 