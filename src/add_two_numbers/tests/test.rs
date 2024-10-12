

#[cfg(test)]
mod test {
    const REPEATES: i32 = 100;


    use rand::prelude::*;
    use super::super::super::solution::*;


    fn rand(offset: i32, range: i32) -> i32 {
        return if range == 0 {
            offset
        } else {
            thread_rng().gen_range((offset)..(offset + range))
        };
    }


    fn test_add_nodes(offset: i32, range: i32)
    {
        let rand0 = rand(offset, range);
        let rand1 = rand(offset, range);
        let sum_real = rand0 + rand1;

        let node0 = new_from_value(rand0);
        let node1 = new_from_value(rand1);

        assert_number_to_node(rand0, &node0);
        assert_number_to_node(rand1, &node1);

        let node_sum = add_nodes(node0, node1);
        let sum_computed = list_to_value(&node_sum).unwrap();

        assert_number_to_node(sum_computed, &node_sum);
        assert!(sum_computed == sum_real);
    }


    fn assert_number_to_node(number: i32, node: &Option<Box<ListNode>>) {
        let number_node = list_to_value(&node).unwrap();
        assert!(number_node == number, "assert_number_to_node failed");
    }


    fn add_nodes(node1: Option<Box<ListNode>>, node2: Option<Box<ListNode>>) -> Option<Box<ListNode>>
    {
        if node1.is_none() || node2.is_none() {
            return None
        }

        let sum = list_to_value(&node1).unwrap() + list_to_value(&node2).unwrap();

        return new_from_value(sum);
    }


    #[test]
    fn test_1()
    {
        for _ in 0..REPEATES {
            test_add_nodes(0, 10);
        }

        println!("test_1 finished");
    }


    #[test]
    fn test_2() {
        for _ in 0..REPEATES {
            test_add_nodes(10, 40);
        }

        println!("test_2 finished");
    }


    #[test]
    fn test_3() {
        for _ in 0..REPEATES {
            test_add_nodes(100, 100);
        }

        println!("test_3 finished");
    }


    #[test]
    fn test_4() {
        for _ in 0..REPEATES {
            test_add_nodes(100, 1000);
        }

        println!("test_4 finished");
    }


    #[test]
    fn test_5() {
        for _ in 0..REPEATES {
            test_add_nodes(0, 1000000);
        }

        println!("test_5 finished");
    }


    #[test]
    fn test_6() {
        test_add_nodes(0, 0);

        println!("test_6 finished");
    }
}