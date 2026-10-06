/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn sort<T: std::cmp::PartialOrd + Copy + std::fmt::Debug>(array: &mut [T]){
    if array.len() <= 1 {
        return;
    }
    println!("array = {:?}", array);
    let pivot = array[0].clone();
    let mut left = 1;
    for right in 1..array.len() {
        if array[right] <= pivot {
            let swap = array[right];
            array[right] = array[left];
            array[left] = swap;
            left += 1;
        }
    }
    let swap = array[left - 1];
    array[left - 1] = pivot;
    array[0] = swap;
    sort(&mut array[..left - 1]);
    if left < array.len() {
        sort(&mut array[left..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
