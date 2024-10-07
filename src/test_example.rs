fn generate_list(n: usize) -> Vec<i32> { 
    (0..n as i32).collect() 
} 

#[cfg(test)] 
mod tests { 
    use super::*; 
    
    #[test] 
    fn test_generate_list_1() { 
        let n = 5; 
        let result = generate_list(n); 
        assert_eq!(result, vec![0, 1, 2, 3, 4]); 
    } 

    #[test] 
    fn test_generate_list_2() { 
        let n = 10; 
        let result = generate_list(n); 
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]); 
    } 

    #[test] 
    fn test_generate_list_3() { 
        let n = 0; 
        let result = generate_list(n); 
        assert_eq!(result, vec![]); 
    } 
}