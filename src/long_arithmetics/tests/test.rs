#[cfg(test)]
mod test {
    use rand::RngCore;
    use crate::long_arithmetics::number::Number;

    #[test]
    fn test_digits_1() {
        test_digits(&[1], &[1], &[1]);
    }

    #[test]
    fn test_digits_2() {
        test_digits(&[1, 1, 1], &[1], &[1, 1, 1]);
    }

    #[test]
    fn test_digits_3() {
        test_digits(&[1, 2, 3], &[0], &[0]);
    }

    #[test]
    fn test_digits_4() {
        test_digits(&[9, 9, 9], &[9, 9, 9], &[9, 9, 8, 0, 0, 1]);
    }

    #[test]
    fn test_integer_to_string_1() {
        test_integer_to_string(1, 1);
    }

    #[test]
    fn test_integer_to_string_2() {
        test_integer_to_string(111, 1);
    }

    #[test]
    fn test_integer_to_string_3() {
        test_integer_to_string(9977, 1234);
    }

    #[test]
    fn test_integer_to_string_4() {
        test_integer_to_string(777999, 8881234);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();

        for i in 0..1000 {
            let v1 = rng.next_u64() as u128;
            let v2 = rng.next_u64() as u128;
            test_integer_to_string(v1, v2);
        }
    }

    #[test]
    fn test_brut_force() {
        let mut rng = rand::thread_rng();

        for v1 in 0..1000 {
            for v2 in 0..1000 {
                test_integer_to_string(v1, v2);
            }
        }
    }

    fn test_digits(v1: &[u8], v2: &[u8], result_checked: &[u8]) {
        let n1 = Number::new_from_array(v1);
        let n2 = Number::new_from_array(v2);

        let result = Number::multiply(&n1, &n2);

        assert_eq!(result.len(), result_checked.len());
        assert!(result.buffer().iter().eq(result_checked.iter().rev()));
    }

    fn test_integer_to_string(v1: u128, v2: u128) {
        let str1 = v1.to_string();
        let str2 = v2.to_string();

        let n1 = Number::new_from_string(&str1);
        let n2 = Number::new_from_string(&str2);

        assert_eq!(n1.as_ref().unwrap().len(), str1.len());
        assert_eq!(n2.as_ref().unwrap().len(), str2.len());

        let mul = Number::multiply(n1.as_ref().unwrap(), n2.as_ref().unwrap());
        let mul_string = mul.to_string();
        let mul_string_reference = (v1 * v2).to_string();

        assert_eq!(mul_string, mul_string_reference, r#"{str1} * {str2} = {mul_string}  correct = {mul_string_reference}"#);
    }
}
