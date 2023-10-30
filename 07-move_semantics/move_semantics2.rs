#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let mut vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {

    vec.push(88);

    vec
}
