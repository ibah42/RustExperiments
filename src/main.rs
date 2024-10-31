mod add_two_numbers;
mod merge_sorted_lists;
mod long_arithmetics;
mod rotate_list;
mod utils;

fn main() {

    add_two_numbers::solution::new_from_value(3);
    merge_sorted_lists::solution::merge_sorted_nodes(&None, &None);
    rotate_list::list_node::ListNode::new(4);

    println!("main finished");
}

