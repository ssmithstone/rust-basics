
#[cfg(test)]
mod tests {
    use super::*;


    mod addition {
        #[test]
        fn test_int_addition(){
            assert_eq!(10 , 5+5);
        }

        #[test]
        fn test_double_addition_with_cast_int_to_float(){
            assert_eq!(10.0 , 5.0 + 5 as f64);
        }

        #[test]
        fn test_double_addition(){
            assert_eq!(10.0 , 5.0+5.0);
        }
    }

    mod subtraction {
        #[test]
        fn test_int_subtraction(){
            assert_eq!(5 , 10 - 5);
        }

        #[test]
        fn test_float_subtraction_with_cast_int_to_float(){
            assert_eq!(5.0 , 10 as f64 - 5.0);
        }
    }

    mod division {

        #[test]
        fn test_int_division(){
            assert_eq!(2 , 10/5);
        }

        #[test]
        fn test_float_division(){
            assert_eq!(2.0 , 10.0/5.0);
        }

        #[test]
        fn test_float_division_with_int_cast_to_float(){
            assert_eq!(2.0 , 10.0/5 as f64);
        }

    }
}
