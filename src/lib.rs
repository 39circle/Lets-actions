use std::ops::Range;

pub fn simple_func(data: i32) -> i32 {
    data
}

pub fn my_func(range: Range<i32>, func: Box<dyn Fn(i32) -> i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in range {
        res.push(func(i));
    }
    res
}

#[cfg(test)]
mod simple_func_test{
    use super::simple_func;
    #[test]
    fn test01() {
        let data = 10;
        let ans = 10;
        let check = simple_func(data);
        assert_eq!(ans, check)
    }
    #[test]
    fn test02() {
        let data = 10;
        let ans = 20;
        let check = simple_func(data);
        assert_ne!(ans, check)
    }
}


#[cfg(test)]
mod my_func_test{
    use super::my_func;
    #[test]
    fn test01() {
        let data = my_func(0..5, Box::new(|x| {
            x*2
        }));
        let ans = vec![0, 2, 4, 6, 8];
        assert_eq!(data, ans)
    }
    #[test]
    fn test02() {
        let data = my_func(0..5, Box::new(|x| {
            x*2
        }));
        let ans = vec![0, 2, 4, 6, 8, 10];
        assert_ne!(data, ans)
    }
}