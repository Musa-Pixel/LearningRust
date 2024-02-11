fn main() {
    let mut arr = Vec::from([23, 23, 1, 90, 12]);
    arr = sort_dac(arr);

    for item in arr {
        println!("{}", item);
    }
}

fn sort_dac(arr: Vec<i32>) -> Vec<i32> {
    let arr_len = arr.len();
    let mut temp: Vec<i32> = Vec::with_capacity(arr_len);
    temp.resize(arr_len, 0);
    
    let (arr, temp) = sort_dac_recursive(arr, temp, 0, arr_len);

    return arr;
}

fn sort_dac_recursive(arr: Vec<i32>, temp: Vec<i32>, start: usize, end: usize) -> (Vec<i32>, Vec<i32>) {
    if start + 1 == end {
        return (arr, temp)
    }
    let middle = (start + end) / 2;
    let (arr, temp) = sort_dac_recursive(arr, temp, start, middle);
    let (mut arr, mut temp) = sort_dac_recursive(arr, temp, middle, end);

    {
        let mut i = start;
        let mut j = middle;
        let mut k = start;
        
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
    (arr, temp)
}
