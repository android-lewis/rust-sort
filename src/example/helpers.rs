pub fn partition<T, F>(v: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        if f(&v[i], &v[last_index]) {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

pub fn swap(param_int_array: &mut [i32], i: usize, j: usize) {
    let temp = param_int_array[i];
    param_int_array[i] = param_int_array[j];
    param_int_array[j] = temp;
}
