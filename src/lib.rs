use std::ops::Range;

/// # simple_func
/// これはシンプルな関数で、与えられた数値を変更することなくそのまま返却します。
pub fn simple_func(data: i32) -> i32 {
    data
}

/// # my_func
/// この関数は、指定された範囲の各値に対して、渡された関数を適用し、その結果を `Vec<i32>` 配列として返却します。 
pub fn my_func(range: Range<i32>, func: Box<dyn Fn(i32) -> i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in range {
        res.push(func(i));
    }
    res
}

/// # nanoda_func
/// なのだをサボるな
/// 詳しくは[これ](https://www.nicovideo.jp/watch/sm42501156?ref=nicoiphone_other)を見ろ
/// 奈良腐女子をなめるな
pub fn nanoda_func(msg: &String) -> String {
    format!("{}なのだ！", msg)
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
#[cfg(test)]
mod nanoda_func_test{
    use super::nanoda_func;
    #[test]
    fn test01() {
        let msg = String::from("こんにちわ");
        let ans = String::from("こんにちわなのだ！");
        let res = nanoda_func(&msg);
        assert_eq!(res, ans)
    }
    #[test]
    fn test02() {
        let msg = String::from("こんにちわ");
        let res = nanoda_func(&msg);
        assert_ne!(res, msg)
    }
}