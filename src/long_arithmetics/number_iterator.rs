use crate::long_arithmetics::number::*;

#[derive(Debug)]
pub struct NumberIterator<'a> {
    current_index: i32,
    number: &'a Number,
    reverse: bool,
}

impl<'a> NumberIterator<'a> {
    pub fn new(reverse: bool, number: &'a Number) -> Self {
        NumberIterator {
            current_index: if reverse { number.len() as i32 - 1 } else { 0 },
            number: number,
            reverse: reverse,
        }
    }
}

impl Iterator for NumberIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let id: i32 = self.current_index;

        if self.reverse {
            if self.current_index < 0 {
                return None;
            }
            self.current_index -= 1
        } else {
            if self.current_index >= self.number.len() as i32 {
                return None;
            }
            self.current_index += 1
        };

        Some(self.number.buffer()[id as usize])
    }
}
