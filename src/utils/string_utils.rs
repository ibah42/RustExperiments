use std::iter::Peekable;
use std::str::Chars;
use num::traits::{Num};

pub fn check_segment_presence_forward(buffer: &mut Peekable<Chars>, mut segment: Peekable<Chars>) -> bool
{
    while segment.peek().is_some() && segment.peek() == buffer.peek() {
        segment.next();
        buffer.next();
    }

    return segment.peek().is_none();
}


pub fn parse_integer_forward<T>(chars: &mut Peekable<Chars>) -> Option<T>
    where T : Num
{
    while chars.peek().is_some() && is_skippable_char(*chars.peek().unwrap()) {
        chars.next();
    }

    let mut integer_str = String::with_capacity(16);

    while chars.peek().is_some() {
        if !chars.peek().unwrap().is_ascii_digit() {
            break;
        }
        integer_str.push( chars.next().unwrap() );
    }

    if integer_str.len() == 0 {
        return None;
    }

    return T::from_str_radix(&integer_str, 10).ok();

    //-----------------------

    fn is_skippable_char(char: char) -> bool { char == '-' || char == '+' || char == ' ' }
}
