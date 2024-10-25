
#[cfg(test)]
mod test {
    use crate::long_arithmetics::number::Number;


    #[test]
    fn test_digits_1() { test_digits(&[1], &[1], &[1]); }


    #[test]
    fn test_digits_2() { test_digits(&[1, 1, 1], &[1], &[1, 1, 1]); }


    #[test]
    fn test_digits_3() { test_digits(&[1, 2, 3], &[0], &[0]); }


    #[test]
    fn test_digits_4() { test_digits(&[9, 9, 9], &[9, 9, 9], &[9, 9, 8, 0, 0, 1]); }

    #[test]
    fn test_integer_to_string_1() { test_integer_to_string(1, 1); }

    #[test]
    fn test_integer_to_string_2() { test_integer_to_string(111, 1); }

    fn test_digits(v1: &[u8], v2: &[u8], result_checked: &[u8]) {
        let n1 = Number::new_from_array(v1);
        let n2 = Number::new_from_array(v2);

        let result = Number::multiply(&n1, &n2);

        assert_eq!(result.len(), result_checked.len());

        for i in 0..result.len() {
            assert_eq!(result.get_buffer()[i], result_checked[i]);
        }
    }

    fn test_integer_to_string(v1: u32, v2: u32){
        let str1 = v1.to_string();
        let str2 = v2.to_string();

        let n1 = Number::new_from_string(&str1);
        let n2 = Number::new_from_string(&str2);

        assert_eq!(n1.as_ref().unwrap().len(), str1.len());
        assert_eq!(n2.as_ref().unwrap().len(), str2.len());

        let mul = Number::multiply(n1.as_ref().unwrap(), n2.as_ref().unwrap());
        let mul_string = mul.to_string();

        assert_eq!(mul_string, (v1 * v2).to_string());
    }
}