// rust で配列をソートするときどうしたらいいんだろう。
pub fn insertion_sort(mut arr: Vec<i64>) -> Vec<i64> {
    for i in 1..arr.len() {
        let mut j = i - 1;
        while j > 0 && arr[j] > arr[i] {
            arr[j + 1] = arr[j];
            j = j - 1
        }
        arr[j + 1] = arr[i];
        println!("{:?}", arr)
    }
    return vec![1, 2, 3, 4, 5, 6];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let input = vec![5, 2, 4, 6, 1, 3];
        let expected = vec![1, 2, 3, 4, 5, 6];
        let actual = insertion_sort(input);

        assert_eq!(actual, expected);
    }
}
