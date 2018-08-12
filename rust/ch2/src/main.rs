use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}

fn select_top(n: usize, mut arr: Vec<i64>) -> Vec<i64> {
    arr.sort();
    arr.reverse();
    arr.iter().take(n).map(|x| *x).collect()
}

const MAX: u64 = 200000;

fn maximux_profit(n: usize, mut arr: Vec<i64>) -> i64 {
    if arr.len() == 0 {
        return 0;
    }

    let mut maxv = -200000;
    let mut minv = arr[0];
    for x in arr.iter().skip(1) {
        maxv = max(maxv, x - minv);
        minv = min(minv, *x)
    }

    return maxv;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_top() {
        let input = vec![25, 36, 4, 55, 71, 18, 71, 89, 65];
        let expected = vec![89, 71, 71];
        let actual = select_top(3, input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_maximu_profit() {
        let input_num = 6;
        let input_arr = vec![5, 3, 1, 3, 4, 3];
        let expected = 3;
        let actual = maximux_profit(input_num, input_arr);

        assert_eq!(actual, expected);
    }
}
