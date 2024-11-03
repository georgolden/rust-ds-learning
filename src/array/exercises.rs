pub fn find_element_arr(arr: &[i32], el: i32) -> i32 {
    if arr.is_empty() {
        return -1;
    }
    for i in 0..arr.len() {
        if arr[i] == el {
            return i.try_into().unwrap();
        }
    }
    return -1;
}
