#[cfg(test)]
mod test {
    use std::thread;
    use std::thread::JoinHandle;
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
        const ITERATIONS : usize = 1024;

        let time_start = std::time::Instant::now();

        for _ in 0..ITERATIONS {
            let v1 = rng.next_u64() as u128;
            let v2 = rng.next_u64() as u128;
            test_integer_to_string(v1, v2);
        }

        println!(r#"{0} ms time per 1 multiplication"#, time_start.elapsed().as_millis() as f32 / ITERATIONS as f32);
    }

    #[test]
    fn test_brut_force() {
        const OFFSET: usize = 1024;

        let threads_count = thread::available_parallelism().unwrap().get() / 2;
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(threads_count);

        let time_start = std::time::Instant::now();

        for i in 0..threads_count {
            let handle: JoinHandle<()> = thread::spawn(move || {
                let low = i * OFFSET;
                let high = (i + 1) * OFFSET;

                for v1 in low..high {
                    for v2 in low..high {
                        test_integer_to_string(v1 as u128, v2 as u128);
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!(r#"{0} ms time per 1 multiplication"#, time_start.elapsed().as_millis() as f32 / OFFSET as f32);
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
