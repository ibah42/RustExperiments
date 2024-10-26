
use core::iter::Iterator;
use std::vec;
use crate::multiply_strings::number_iterator::NumberIterator;


#[derive(Debug)]
pub struct Number {
    digits: Vec<u8>,
}


impl Clone for Number {
    fn clone(&self) -> Self {
        return Number { digits: self.digits.clone() };
    }
}

impl Number {

    pub fn get_digits(&self) -> &Vec<u8> { &self.digits }

    pub fn iter(&self) -> NumberIterator {
        NumberIterator::new( false, &self )
    }

    pub fn iter_reverse(&self) -> NumberIterator {
        NumberIterator::new( true, &self )
    }

    pub fn new_from_string(s: &str) -> Option<Self> {
        let mut digits: Vec<u8> = Vec::with_capacity(s.len());

        for c in s.chars().rev() {
            if !c.is_digit(10) {
                return None;
            }

            digits.push(c.to_digit(10).unwrap() as u8);
        }

        return Some(Number { digits: digits });
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::with_capacity(self.digits.len());
        for n in self.iter_reverse() {
            result.push_str(&n.to_string());
        }
        result
    }

    pub fn new_from_array(a: &[u8]) ->Self {
        Number { digits: a.to_vec() }
    }

    pub fn len (&self) -> usize {self.digits.len()}

    pub fn multiply(n1: &Number, n2 : &Number) -> Self {
        let mut array : Vec<u8> = Vec::with_capacity( n1.len() + n2.len() );
        let mut global_offset : i32 = -1;

        for v1 in n1.iter_reverse() {
            global_offset += 1;

            let mut offset = global_offset;
            let mut reminder: u8 = 0;

            for v2 in n2.iter_reverse() {
                let mut multiplication: u8 = v1 * v2 + reminder;

                if array.len() as i32 <= offset {
                    reminder = multiplication / 10;
                    multiplication %= 10;

                    array.push(multiplication);
                } else {
                    multiplication += array[offset as usize];
                    reminder = multiplication / 10;
                    multiplication %= 10;

                    array[offset as usize] = multiplication;
                }

                offset += 1;
            }

            if reminder != 0 {
                if array.len() as i32 <= offset {
                    array.push(reminder);
                } else {
                    array[offset as usize] += reminder;
                }
            }
        }

        let mut result = Number { digits: array };
        result.normalize();
        return result;
    }
    
    fn new_from_number_multiply(number: &Number, mul: u8) -> Self {
        let mut array = Vec::with_capacity(number.len() + 1);

        let mut reminder = 0;
        for n in number.iter() {
            let total = n * mul + reminder;
            array.push( total % 10 );
            reminder = total / 10;
        }

        Number { digits: array }
    }

    fn normalize(&mut self) {
        let mut first_nulls : usize = 0;
        for v in self.iter() {
            if v != 0 {
                break;
            }
            first_nulls += 1;
        }

        if first_nulls >= self.len() {
            self.digits = vec![0; 1];
            return;
        }

        if first_nulls != 0 {
            self.digits  = Vec::from_iter(self.digits[..first_nulls].iter().cloned());    
        }
    }

}
