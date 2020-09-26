use crate::helpers::partition;
use crate::helpers::swap;

pub fn do_hard_work(sort: &String) {
        println!("{}", sort);
        let mut array:[i32; 6] = [2, 4, 6, 12, 55, 13];
        let high = array.len() - 1;
        let n = array.len();
        //quick_sort(&mut array, 0, high);
        bubble_sort(&mut array, n);
        println!("{:?}", array);
    }

    pub fn quick_sort(array: &mut [i32], low: usize, high: usize) {
        
        if low < high {
            let pi = partition(array, low, high);
            quick_sort(array, low, (pi - 1) as usize);
            quick_sort(array, (pi + 1) as usize, high);
        }
        
    }

    

    pub fn bubble_sort(array: &mut [i32], n: usize) {
        let mut swapped = false;

        for i in 0..(n-1) {
            swapped = false;
            for j in 0..(n-i-1) {
                if (array[j] > array[j + 1])  
                { 
                    swap(array, j, j+1);
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