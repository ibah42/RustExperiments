
use core::iter::Iterator;
use std::cmp::max;
use std::vec;
use crate::long_arithmetics::number_iterator::NumberIterator;


#[derive(Debug)]
pub struct Number {
    digits: Vec<u8>,
}


impl Clone for Number {
    #[inline]
    fn clone(&self) -> Self {
        return Number { digits: self.digits.clone() };
    }
}

impl Number {

    #[inline]
    pub fn new_zero() -> Self {
        Number { digits: vec![0; 1] }
    }

    pub fn new_from_array(a: &[u8]) -> Self {
        assert!(a.len() > 0);
        Number { digits: a.iter().rev().cloned().collect::<Vec<u8>>() }
    }

    pub fn new_from_string(s: &str) -> Option<Self> {
        let mut digits: Vec<u8> = Vec::with_capacity(s.len());

        for c in s.chars().rev() {
            if !c.is_digit(10) {
                return None;
            }

            digits.push(c.to_digit(10).unwrap() as u8);
        }

        if digits.len() == 0 {
            Some(Number::new_zero())
        } else {
            Some(Number { digits: digits })
        }
    }

    #[inline]
    pub fn buffer(&self) -> &Vec<u8> { &self.digits }

    #[inline]
    pub fn buffer_mut(&mut self) -> & mut Vec<u8> { & mut self.digits }

    #[inline]
    pub fn iter(&self) -> NumberIterator { NumberIterator::new(false, &self) }

    #[inline]
    pub fn iter_reverse(&self) -> NumberIterator { NumberIterator::new(true, &self) }

    #[inline]
    pub fn len(&self) -> usize { self.digits.len() }

    pub fn to_string(&self) -> String {
        let mut result: String = String::with_capacity(self.digits.len());
        for n in self.iter_reverse() {
            result.push_str(&n.to_string());
        }
        result
    }

    pub fn try_get_digit(&self, index: usize) -> u8 {
        if index >= self.len() {
            0
        } else {
            self.digits[index]
        }
    }

    pub fn multiply(number1: &Number, number2: &Number) -> Self {
        let mut result = Number::new_zero();
        let mut offset: i32 = 0;

        for n1 in number1.iter() {
            let mul_byte_result = Self::multiply_byte(number2, n1, offset);
            result = Self::sum(&result, &mul_byte_result);

            offset += 1;
        }

        result.normalize_zeros();
        return result;
    }

    pub fn sum(number1: &Number, number2: &Number) -> Self {
        let max_id = max(number1.len(), number2.len());
        let mut array = Vec::with_capacity(max_id + 1);
        let mut reminder = 0;

        for id in 0..max_id {
            let total = reminder + number1.try_get_digit(id) + number2.try_get_digit(id);
            array.push(total % 10);
            reminder = total / 10;
        }

        if reminder != 0 {
            array.push(reminder);
        }

        Number { digits: array }
    }

    fn normalize_zeros(&mut self) {
        let mut new_len = self.len();

        for i in (0..self.len()).rev() {
            if self.try_get_digit(i) == 0 {
                new_len = i;
            } else {
                break;
            }
        }

        if new_len != self.len() {
            if new_len == 0 {
                new_len = 1;
            }

            self.digits.truncate(new_len);
        }
    }

    fn multiply_byte(number: &Number, mul: u8, offset: i32) -> Self {
        if mul == 0 {
            return Number::new_zero();
        }

        let mut array = Vec::with_capacity(number.len() + 1 + offset as usize);
        for _ in 0..offset {
            array.push(0);
        }

        let mut reminder = 0;

        for n in number.iter() {
            let total = n * mul + reminder;
            array.push(total % 10);
            reminder = total / 10;
        }

        if reminder != 0 {
            array.push(reminder);
        }

        Number { digits: array }
    }

}
