use crate::example::helpers;
use crate::example::sort_types;
use std::convert::TryInto;

pub fn do_hard_work(sort: &sort_types::Sort) {
    println!("{:?}", sort);
    let mut array = [1, 0, 9, 8, 100, 345, 5, 6, 7, 4, 3, 2, 1];
    println!("Before: {:?}\n", array);
    let n: usize = array.len();
    //let high = array.len() - 1;
    match sort {
        sort_types::Sort::Quick => quick_sort(&mut array, &|x, y| x > y),
        sort_types::Sort::Bubble => bubble_sort(&mut array, n),
        sort_types::Sort::Merge => merge_sort(&mut array, 0, n - 1),
        sort_types::Sort::Insertion => insertion_sort(&mut array, n),
        sort_types::Sort::Selection => bubble_sort(&mut array, n),
        sort_types::Sort::Heap => bubble_sort(&mut array, n),
        sort_types::Sort::Radix => bubble_sort(&mut array, n),
        sort_types::Sort::Bucket => bubble_sort(&mut array, n),
    }
    println!("After: {:?}\n", array);
}

pub fn quick_sort<T, F>(array: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = array.len();
    if len >= 2 {
        let pivot_index = helpers::partition(array, f);
        quick_sort(&mut array[0..pivot_index], f);
        quick_sort(&mut array[pivot_index + 1..len], f);
    }
}

pub fn bubble_sort(array: &mut [i32], n: usize) {
    for i in 0..(n - 1) {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

pub fn merge_sort(array: &mut [i32], l: usize, r: usize) {
    if l < r {
        let m = (l + (r - 1)) / 2;
        merge_sort(array, l, m);
        merge_sort(array, m + 1, r);
        helpers::merge(array, l, m, r);
    }
}

pub fn insertion_sort(array: &mut [i32], n: usize) {
    for i in 1..n {
        let key = array[i];
        let mut j: isize = (i - 1).try_into().unwrap();
        while j >= 0 && key < array[j as usize] {
            array[(j + 1) as usize] = array[j as usize];
            j -= 1;
        }
        array[(j + 1) as usize] = key;
    }
}
/*
pub fn selection_sort(start: &[i32]) {

}

pub fn heap_sort(start: &[i32]) {

}

pub fn radix_sort(start: &[i32]) {

}

pub fn bucket_sort(start: &[i32]) {

}

*/
