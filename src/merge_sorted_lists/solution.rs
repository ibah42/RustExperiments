
/*
    Task-2

    https://leetcode.com/problems/merge-two-sorted-lists/
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
        ListNode { next: None, value }
    }

    #[inline]
    pub fn new_list(value: i32, next_node: Option<Box<ListNode>>) -> Self {
        ListNode { next: next_node, value }
    }

    pub fn deep_clone(node: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if node.is_none() {
            return None;
        }
        let next_node = ListNode::deep_clone(&node.as_ref()?.next);
        return Some(Box::new(ListNode::new_list(node.as_ref()?.value, next_node)));
    }

    pub fn len(mut node: &Option<Box<ListNode>>) -> usize {
        let mut result = 0;
        while node.is_some() {
            result += 1;
            node = &node.as_ref().expect("null node").next;
        }

        return result;
    }


    #[allow(dead_code)]
    #[inline]
    fn is_empty(node0: &Option<Box<ListNode>>, node1: &Option<Box<ListNode>>) -> bool {
        node0.is_none() && node1.is_none()
    }


    #[allow(dead_code)]
    #[inline]
    fn is_full(node0: &Option<Box<ListNode>>, node1: &Option<Box<ListNode>>) -> bool{
        node0.is_some() && node1.is_some()
    }


    #[allow(dead_code)]
    #[inline]
    fn only_one_full<'a>(node0: &'a Option<Box<ListNode>>, node1: &'a Option<Box<ListNode>>) -> &'a Option<Box<ListNode>>{
        debug_assert_ne!(node0.is_some(), node1.is_some());
        return if node0.is_some() { node0 } else { node1 };
    }
}


pub fn merge_sorted_nodes(node0: &Option<Box<ListNode>>, node1: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if node0.is_none() || node1.is_none() {
        return ListNode::deep_clone(ListNode::only_one_full(node0, node1));
    }

    let mut node0_iter = node0;
    let mut node1_iter = node1;

    let mut result: Option<Box<ListNode>> = create_new_from_selected_and_iterate(&mut node0_iter, &mut node1_iter);
    let mut node_iterator = &mut result;

    while ListNode::is_full(node0_iter, node1_iter) {
        node_iterator.as_mut()?.next = create_new_from_selected_and_iterate(&mut node0_iter, &mut node1_iter);
        node_iterator = &mut node_iterator.as_mut()?.next;
    }

    if node0_iter.is_some() != node1_iter.is_some() {
        let node_remainder = ListNode::only_one_full(node0_iter, node1_iter);
        node_iterator.as_mut()?.next = ListNode::deep_clone(node_remainder);
    }
    return result;
}


fn create_new_from_selected_and_iterate(
    node0: &mut &Option<Box<ListNode>>,
    node1: &mut &Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    let value : i32;
    assert!(ListNode::is_full(*node0, *node1));

    if node0.as_ref()?.value <= node1.as_ref()?.value {
        value = node0.as_ref()?.value;
        *node0 = &node0.as_ref()?.next;
    } else {
        value = node1.as_ref()?.value;
        *node1 = &node1.as_ref()?.next;
    }

    return Some(Box::new(ListNode::new(value)));
}

