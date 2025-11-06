fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    let mut v = Vec::new();
    for i in a {
        v.push(i);
    }

    (a, v)
}

fn main() {

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", v.get(10));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
