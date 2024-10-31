
/*

   cargo test --release --no-fail-fast -- --test-threads=1 tests

 */

#[cfg(test)]
mod test {
    use crate::rotate_list::list_node::ListNode;

    fn test(input : &str, output : &str, separator:&str, rotations: i32) {
        let mut root = ListNode::new_from_string(input, separator);

        println!("{0}", ListNode::print(&root, Some(separator)));

        root = ListNode::rotate(root, rotations);

        let debug_list_str = ListNode::print(
            &root,
            if output.contains(separator) { Some(separator) } else { None }
        );

        assert!(
            debug_list_str == output ||
            debug_list_str == output.to_string() + separator
        );
    }

    #[test]
    fn test_1() {
        test("0,1,2,3,4,5,6,7,8,9", "7890123456", ",", 3);
    }

    #[test]
    fn test_2() {
        for i in 0..100 {
            test("0,1,2,", "0,1,2", ",", i * 3);
        }
    }
}