extern crate string_builder;
use string_builder::Builder;
use crate::utils::string_utils;

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

    pub fn new_from_string(str: &str, separator: &str) -> Option<Box<ListNode>>
    {
        let mut str_iterator = str.chars().peekable();
        let mut integer = string_utils::parse_integer_forward::<i32>(&mut str_iterator);

        //println!("char {0}", str_iterator.clone().peekable().peek().unwrap());

        if integer.is_none(){
            return None;
        }

        let mut root = Some(Box::new(ListNode::new(integer.unwrap())));
        let mut node = &mut root;

        while string_utils::check_segment_presence_forward(&mut str_iterator, separator.chars().peekable())
        {
            integer = string_utils::parse_integer_forward::<i32>(&mut str_iterator);
            if integer.is_none() {
                break;
            }

            node.as_mut().unwrap().next = Some(Box::new(ListNode::new(integer.unwrap())));
            node = &mut node.as_mut().unwrap().next;
        }

        return root;
    }

    pub fn new_from_array(array: &[i32]) -> Option<Box<ListNode>>
    {
        if array.is_empty() {
            return None;
        }

        let mut root = Box::new(ListNode::new(array[0]));
        let mut node = &mut root;

        for i in 1..array.len() {
            node.next = Some(Box::new(ListNode::new(array[i])));
            node = node.next.as_mut().unwrap();
        }

        return Some(root);
    }

    pub fn len(mut node: &Option<Box<ListNode>>) -> usize {
        let mut result = 0;
        while node.is_some() {
            result += 1;
            node = &node.as_ref().unwrap().next;
        }

        return result;
    }

    pub fn print(mut node: &Option<Box<ListNode>>, separator: Option<& str> ) -> String {
        let mut builder = Builder::new(ListNode::len(&node) * 16);

        while node.is_some() {
            builder.append(node.as_ref().unwrap().value.to_string().as_bytes());
            if separator.is_some() {
                builder.append(*separator.as_ref().unwrap());
            }
            node = &node.as_ref().unwrap().next;
        }

        return builder.string().unwrap();
    }

    pub fn last(root: &mut Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        let mut node = root;

        while node.is_some() &&
            node.as_ref().unwrap().next.is_some()
        {
            node = &mut node.as_mut().unwrap().next;
        }

        return node;
    }

    pub fn split_list(id: i32, mut root: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if id == 0 {
            panic!("id == 0");
        }

        let mut node = &mut root;
        let mut iterations = id - 1;

        while node.is_some() && iterations > 0 {
            iterations -= 1;
            node = &mut node.as_mut().unwrap().next;
        }

        if iterations > 0 {
            panic!("{iterations} is out of range [1..len)");
        }

        let second_root = node.as_mut().unwrap().next.take();
        return (root, second_root);
    }

    pub fn rotate(mut root: Option<Box<ListNode>>, mut iterations: i32) -> Option<Box<ListNode>> {
        if iterations <= 0 {
            return root;
        }

        let root_len = ListNode::len(&root);
        iterations %= root_len as i32;

        if iterations == 0 {
            return root;
        }

        let moves = root_len as i32 - iterations;
        let (mut new_root1, mut new_root2) = ListNode::split_list(moves, root);
        ListNode::last(&mut new_root2).as_mut().unwrap().next = new_root1;

        return new_root2;
    }
}
