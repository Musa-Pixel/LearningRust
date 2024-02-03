fn main() {
    let mut arr = Vec::from([23, 23, 1, 90, 12]);
    sort_dac(&mut arr);

    for item in arr {
        println!("{}", item);
    }
}

fn sort_dac(arr: &mut Vec<i32>) {
    let arr_len = arr.len();
    let mut temp: Vec<i32> = Vec::with_capacity(arr_len);
    temp.resize(arr_len, 0);
    
    sort_dac_recursive(arr, &mut temp, 0, arr_len);
}

fn sort_dac_recursive(arr: &mut Vec<i32>, temp: &mut Vec<i32>, start: usize, end: usize) {
    if start + 1 == end {
        return
    }
    let middle = (start + end) / 2;
    sort_dac_recursive(arr, temp, start, middle);
    sort_dac_recursive(arr, temp, middle, end);

    {
        let mut i = start;
        let mut j = middle;
        let mut k: usize = start;
        
        while i < middle && j < end {
            if arr[i] < arr[j] {
                temp[k] = arr[i];
                i += 1;
                k += 1;
            }
            else {
                temp[k] = arr[j];
                j += 1;
                k += 1;
            }
        }
        while i < middle {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }
        while i < middle {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }
    }

    for i in start..end {
        arr[i] = temp[i];
    }
}
