use crate::example::helpers;
use crate::example::sort_types;

pub fn do_hard_work(sort: &sort_types::Sort) {
    println!("{:?}", sort);
    let mut array = [1, 0, 9, 8, 100, 345, 5, 6, 7, 4, 3, 2, 1];
    let n: usize = array.len();
    //let high = array.len() - 1;
    match sort {
        sort_types::Sort::Quick => quick_sort(&mut array, &|x, y| x < y),
        sort_types::Sort::Bubble => bubble_sort(&mut array, n),
        sort_types::Sort::Merge => bubble_sort(&mut array, n),
        sort_types::Sort::Insertion => bubble_sort(&mut array, n),
        sort_types::Sort::Selection => bubble_sort(&mut array, n),
        sort_types::Sort::Heap => bubble_sort(&mut array, n),
        sort_types::Sort::Radix => bubble_sort(&mut array, n),
        sort_types::Sort::Bucket => bubble_sort(&mut array, n),
    }
    println!("After:  {:?}\n", array);
}

pub fn quick_sort<T, F>(v: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = helpers::partition(v, f);
        quick_sort(&mut v[0..pivot_index], f);
        quick_sort(&mut v[pivot_index + 1..len], f);
    }
}

pub fn bubble_sort(array: &mut [i32], n: usize) {
    for i in 0..(n - 1) {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if array[j] > array[j + 1] {
                helpers::swap(array, j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
/*
pub fn merge_sort(start: &[i32]) {

}

pub fn insertion_sort(start: &[i32]) {

}

pub fn selection_sort(start: &[i32]) {

}

pub fn heap_sort(start: &[i32]) {

}

pub fn radix_sort(start: &[i32]) {

}

pub fn bucket_sort(start: &[i32]) {

}

*/
