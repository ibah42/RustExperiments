
/*
    Task-1

    https://leetcode.com/problems/add-two-numbers/description/
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub value: i32,
    pub next: Option<Box<ListNode>>
}


impl ListNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        assert!(value >= 0 && value < 10);
        ListNode { next: None, value }
    }
}


pub fn new_from_value(number: i32) -> Option<Box<ListNode>> {
    if number < 0 {
        return None;
    }

    let mut processed = number;
    let mut root = Box::new(ListNode::new(processed % 10));
    processed /= 10;
    let mut iteration_node = &mut root;

    while processed > 0 {
        let new_node = Box::new(ListNode::new(processed % 10));
        processed /= 10;
        iteration_node.next = Some(new_node);
        iteration_node = iteration_node.next.as_mut().unwrap();
    }

    Some(root)
}


pub fn list_to_value(input_node: &Option<Box<ListNode>>) -> Option<i32>
{
    if input_node.is_none() {
        return None;
    }

    let mut iteration_node: &Option<Box<ListNode>> = input_node;
    let mut result: i32 = 0;
    let mut multiplier: i32 = 1;

    while iteration_node.is_some()
    {
        match iteration_node {
            None => break,
            Some(node_value) => {
                result += multiplier * node_value.value;
                iteration_node = &node_value.next;
            }
        }

        multiplier *= 10;
    }

    return Some(result);
}
