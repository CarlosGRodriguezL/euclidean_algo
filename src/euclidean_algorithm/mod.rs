pub mod eucl_algo_recursive;
pub mod eucl_algo_loop;

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;

    const TEST_CASES: [(u64, u64, u64); 7] = [
        (10, 25, 5),
        (14, 28, 14),
        (1234567890123456789, 9876543210987654321, 90000000009),
        (18446744073709551615, 9223372036854775807, 1),
        (18446744073709551614, 18446744073709551613, 1),
        (18446744073709551557, 18446744073709551559, 1),
        (18446744073709551557, 18446744073709551558, 1),
    ];

    #[test]
    fn eucl_algo_recursive_test() { 
        let mut i = 0;
        while i < TEST_CASES.len() {
            let (mut a, mut b, expected) = TEST_CASES[i];
    
            if b > a {
                (a, b) = (b, a);
            }

            let result = eucl_algo_recursive::run(a, b);
            assert_eq!(result, expected);
            i = i + 1;
        }
    }

    #[test]
    fn eucl_algo_loop_test() {
        let mut i = 0;
        while i < TEST_CASES.len() {
            let (mut a, mut b, expected) = TEST_CASES[i];
        
            if b > a {
                (a, b) = (b, a);
            }

            let result = eucl_algo_loop::run(a, b);

            println!("{a} {b} {result} {expected}");
            assert_eq!(result, expected);
        
            i = i + 1;
        }
    }

    #[test]
    fn eucl_algo_loop_bigint_test() {
        let mut i = 0;
        while i < TEST_CASES.len() {
            let (mut a, mut b, expected) = TEST_CASES[i];
        
            if b > a {
                (a, b) = (b, a);
            }

            let result = eucl_algo_loop::run_bigint(
                FromPrimitive::from_u64(a).unwrap(), 
                FromPrimitive::from_u64(b).unwrap(),
            );

            println!("{a} {b} {result} {expected}");
            assert_eq!(result, FromPrimitive::from_u64(expected).unwrap());
        
            i = i + 1;
        }
    }

    #[test]
    fn eucl_algo_recursive_bigint_test() {
        let mut i = 0;
        while i < TEST_CASES.len() {
            let (mut a, mut b, expected) = TEST_CASES[i];
        
            if b > a {
                (a, b) = (b, a);
            }

            let result = eucl_algo_recursive::run_bigint(
                &FromPrimitive::from_u64(a).unwrap(), 
                &FromPrimitive::from_u64(b).unwrap(),
            );

            println!("{a} {b} {result} {expected}");
            assert_eq!(result, FromPrimitive::from_u64(expected).unwrap());
        
            i = i + 1;
        }
    }

}
