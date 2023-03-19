fn good_add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

mod tests {
    use super::*;

    #[test]
    fn test_good_add() {
        assert_eq!(good_add(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
