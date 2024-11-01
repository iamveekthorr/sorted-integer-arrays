fn main() {
    let ans = sorted_integer_arrays(&[1, 2, 3, 4, 5], &[6, 7, 8, 10]);
    println!("{:?}", ans)
}

fn sorted_integer_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut i = 0; // starting index
    let mut j = 0; // ending index

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            ans.push(arr1[i]);
            i += 1; // increment array index for first array
        } else {
            ans.push(arr2[j]);
            j += 1; // increment array index for second array
        }
    }

    // Make sure the array is completely exhausted
    while i < arr1.len() {
        ans.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        ans.push(arr2[j]);
        j += 1;
    }

    return ans;
}
