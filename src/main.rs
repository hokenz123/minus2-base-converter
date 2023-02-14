fn to_base2(mut num: u32) -> u32 {
    let mut res: String = "".to_owned();
    while num != 0 {
        res = format!("{0}{1}", res, num%2);
        num /= 2;
    }
    res = res.chars()
        .rev()
        .collect::<String>();
    res.parse().unwrap()
}

fn from_minus2(num: u32) -> i32 {
    let mut res: i32 = 0;
    let str_primary = num.to_string();
    let mut str: &str = &str_primary[..];
    let mut i = 0u32;
    let rev_str = str.chars()
                     .rev()
                     .collect::<String>();
    str = &rev_str;
    while i != str.chars()
                  .count()
                  .try_into()
                  .unwrap() {
        let g = str.bytes().nth(i.try_into()
                                            .unwrap())
                                            .unwrap();
        if g == 49 {
            res += i32::pow(-2, i);
        }
        i += 1;
    }
    res
}

fn to_minus2(num: i32) -> u32 {
    let mut res = "".to_string();
    let mut value = num;

    while value != 0 {
        let mut rem = value % -2;
        value /= -2;
        if rem < 0 {
            rem += 2;
            value += 1;
        }
        if rem == 1 {
            res = format!("1{}", res);
        }
        if rem == 0 {
            res = format!("0{}", res);
        }
    }

    res.parse().unwrap()
}

fn main() {}