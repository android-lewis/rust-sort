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

pub fn merge(array: &mut [i32], l: usize, m: usize, r: usize) {
    let n1: usize = m - l + 1;
    let n2 = r - m;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for i in 0..n1 {
        left.push(array[l + i]);
    }
    for j in 0..n2 {
        right.push(array[m + 1 + j]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < n1 && j < n2 {
        if left[i] <= right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        array[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        array[k] = right[j];
        j += 1;
        k += 1;
    }
}
