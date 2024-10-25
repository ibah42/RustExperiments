
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
    pub fn get_buffer(&self) -> &Vec<u8> { &self.digits }

    #[inline]
    pub fn iter(&self) -> NumberIterator {
        NumberIterator::new(false, &self)
    }

    #[inline]
    pub fn iter_reverse(&self) -> NumberIterator {
        NumberIterator::new(true, &self)
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::with_capacity(self.digits.len());
        for n in self.iter_reverse() {
            result.push_str(&n.to_string());
        }
        result
    }

    #[inline]
    pub fn len(&self) -> usize { self.digits.len() }

    pub fn multiply(number1: &Number, number2: &Number) -> Self {
        let mut result = Number::new_zero();
        let mut offset: i32 = 0;

        for n1 in number1.iter() {
            let mul_byte_result = Self::multiply_byte(number2, n1, offset);
            result = Self::sum(&result, &mul_byte_result);

            offset += 1;
        }

        return result;
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

        Number { digits: array }
    }

    pub fn sum(number1: &Number, number2: &Number) -> Self {
        let mut array = Vec::with_capacity(max(number1.len(), number2.len()) + 1);
        let mut reminder = 0;

        for n1 in number1.iter() {
            for n2 in number2.iter() {
                let total = reminder + n1 + n2;
                array.push(total % 10);
                reminder = total / 10;
            }
        }
        Number { digits: array }
    }
}
