mod add_two_numbers;
mod merge_sorted_lists;

fn main() {

    add_two_numbers::solution::new_from_value(3);
    merge_sorted_lists::solution::merge_sorted_nodes(&None, &None);

    println!("main finished");
}

