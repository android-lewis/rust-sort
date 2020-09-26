    pub fn partition(array: &mut[i32], low: usize, high: usize) -> i32 {
        let pivot = array[high];
        let mut i = low;
        let mut j = low;
    
        while j < high {
            if array[j] < pivot {
                swap(array, j, i);
                i += 1;
            }
            j += 1;
        }
       swap(array, i, high);
       return i as i32;
    }
    
    pub fn swap(param_int_array: &mut [i32], i: usize, j: usize ) {
       let temp = param_int_array[i];
       param_int_array[i] = param_int_array[j];
       param_int_array[j] = temp;
    }
