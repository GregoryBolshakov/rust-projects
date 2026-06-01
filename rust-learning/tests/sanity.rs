// Integration test sanity check.
// Run with: cargo test --test sanity

#[test]
fn addition_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn vec_iteration_works() {
    let xs: Vec<i32> = (1..=3).collect();
    let doubled: Vec<i32> = xs.iter().map(|n| n * 2).collect();
    assert_eq!(doubled, [2, 4, 6]);
}
