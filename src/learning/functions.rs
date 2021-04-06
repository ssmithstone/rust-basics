fn no_params_no_return_type(){
    println!("Hello World");
}

fn copy_by_value_int(value : i32 ){
    println!("value {}" , value);

}

fn copy_by_ref_strings(value: & String){
    println!("value {}" , *value);

}

fn copy_by_ref_str(value: &str){
    println!("value {}" , value);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplest_fn(){
        no_params_no_return_type();
    }

    #[test]
    fn test_copy_value_for_primitives_fn(){
        copy_by_value_int(10);
    }

    #[test]
    fn test_copy_ref_for_fn(){
        let v = "joe bloc";
        copy_by_ref_str(v);
    }

}
