fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // Use y to modify x
    let z = &mut x; // Create a new mutable reference after y is done
    *z += 1000; // Use z to modify x
    assert_eq!(x, 1200);
}