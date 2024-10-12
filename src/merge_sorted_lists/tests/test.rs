
#[cfg(test)]
mod test {
    use crate::merge_sorted_lists::solution;
    use crate::merge_sorted_lists::solution::ListNode;


    #[test]
    fn test1() {
        test_template(&[1, 2, 3], &[4, 5, 6], &[1, 2, 3, 4, 5, 6], "test 1 finished");
    }

    #[test]
    fn test2() {
        test_template(&[4, 5, 6], &[1, 2, 3], &[1, 2, 3, 4, 5, 6], "test 2 finished");
    }

    #[test]
    fn test3() {
        test_template(&[1, 3, 5], &[2, 4, 6], &[1, 2, 3, 4, 5, 6], "test 3 finished");
    }

    #[test]
    fn test4() {
        test_template(&[1, 3, 5], &[2, 4, 6, 7, 10], &[1, 2, 3, 4, 5, 6,  7, 10], "test 4 finished");
    }

    #[test]
    fn test5() {
        test_template(
            &[1, 1, 1, 2, 2, 10, 11],
            &[1, 2, 3, 3, 20, 30, 40],
            &[1, 1, 1, 1, 2, 2, 2, 3, 3, 10, 11, 20, 30, 40],
            "test 5 finished");
    }



    fn test_template(array0: &[i32], array1: &[i32], array_result: &[i32], message: &str)
    {
        let chain0 = create_list_node_chain(array0);
        let chain1 = create_list_node_chain(array1);

        let chain_result = solution::merge_sorted_nodes(&chain0, &chain1);

        assert_chain_equals_to(array_result, &chain_result);

        println!("{}", message);
    }


    fn assert_chain_equals_to(array: &[i32], chain: &Option<Box<ListNode>>)
    {
        assert!(array.len() == ListNode::len(chain));

        let mut node = chain;
        for i in 0..array.len()
        {
            assert!(array[i] == node.as_ref().expect("").value);
            node = &node.as_ref().expect("").next;
        }
    }


    fn create_list_node_chain(numbers: &[i32]) -> Option<Box<ListNode>> {
        if numbers.is_empty() {
            return None;
        }

        let mut result = Some(Box::new(ListNode::new(numbers[0])));
        let mut node: &mut Option<Box<ListNode>> = &mut result;

        for i in 1..numbers.len() {
            let number: i32 = numbers[i];
            let number_nonstandard = * numbers.get(i).unwrap();
            assert!(number == number_nonstandard);
            node.as_mut()?.next = Some(Box::new(ListNode::new(number)));
            node = &mut node.as_mut()?.next;
        }

        assert_chain_equals_to(numbers, &result);
        return result;
    }
}