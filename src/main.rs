use std::cmp::min;

fn main() {
    println!("{:?}", merge_sort(& mut[3,3,3,3,4,1,2,3,5,7,6]));
    println!("{:?}", merge_sort(& mut[4,1,2]));
    println!("{:?}", merge_sort(& mut[-4,0,0,-82]));
}

fn merge_sort(array: & mut[i32]) -> &[i32] {
    let n: usize = array.len();
    if n < 2 {
        return array;
    }

    // else
    let mut working: Vec<i32> = vec![0; n];
    let mut width: usize = 1;
    while width < n {
        let mut left: usize = 0;
        while left < n {
            let middle: usize = min(left + width, n - 1);
            let end: usize = min(left + (width * 2), n);
            merge(&mut working[left..end], &array[left..middle], &array[middle..end]);
            array[left..end].copy_from_slice(&working[left..end]);
            left += width*2;
        }
        // Increasing sub array size by powers of 2
        width *= 2;
    }
    return array;
}

fn merge(working: &mut[i32], left: &[i32], right: &[i32]){
    let left_size: usize = left.len();
    let right_size: usize = right.len();
    // Only one list, nothing to merge
    if right_size == 0 {
        return;
    }
    // Already sorted
    if left.last() <= right.first() {
        let (left_working, right_working) = working.split_at_mut(left_size);
        left_working.copy_from_slice(left);
        right_working.copy_from_slice(right);
        return;
    }
    let mut index: usize = 0;
    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    while left_index < left_size && right_index < right_size {
        let left_val = left[left_index];
        let right_val = right[right_index];
        if left_val <= right_val{
            working[index] = left_val;
            left_index += 1;
        }
        else {
            working[index] = right_val;
            right_index += 1;
        }
        index += 1;
    }

    if index < working.len() {
        // We reached the end of one list, copy any remaining elements from the other list
        let (_sorted, remaining) = working.split_at_mut(index);
        if left_index < left_size {
            remaining.copy_from_slice(&left[left_index..]);
        }
        else if right_index < right_size {
            remaining.copy_from_slice(&right[right_index..]);
        }
    }
}
