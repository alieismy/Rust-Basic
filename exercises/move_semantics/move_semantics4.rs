fn main() {
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` now creates and returns a new Vec<i32>
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // Create a new vector inside the function
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // Return the vector
}