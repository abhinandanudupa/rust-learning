fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // Use the vector macro.
    // let v = ???;
    // let v = Vec::from(a);
    // let v = a.to_vec();
    let mut v = vec![];
    for i in a.into_iter() {
        v.push(i);
    }


    (a, v)
}

fn main() {
    // You can optionally experiment here.
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
