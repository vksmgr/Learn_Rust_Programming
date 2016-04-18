pub fn insertion_sort(arr: &mut[i32]) ->&mut [i32]{
    let mut i =0;
    let mut j =1;
    let arr_len = arr.len();
    let mut key =0;


    for i in 1..arr_len{
        key = arr[i];
        j = i-1;
        while j>= 0 && key < arr[j] {
        arr[j+1] = arr[j];
            j = j-1;
        }
        arr[j + 1] = key;
    }
    arr
}

/*
pub fn bubble_sort(arr: &mut[i32]) ->&mut [i32]{
    let mut i = 0;
    let mut swapped: bool = true;
    let mut j = 0;
    let mut temp = 0;
    let mut arr_len = arr.len();
    while swapped {
        swapped = false;
        j = j+1;
        for x in i < (arr_len-j){

            if arr[i] > arr[i+1]{
                temp = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = temp;
                swapped = true;
            }
            i = i+1;
        }
    }

    arr
}
*/

pub fn selection_sort(arr: &mut[i32]) ->&mut [i32]{
    let mut i = 0;
    let mut j = 0;
    let mut min = 0;
    let mut temp = 0;
    let mut arr_len = arr.len();

    while i<arr_len-1 {

        min = i;
        j = i+1;
        while j<arr_len {

            if arr[j] < arr[min]{
                min = j;
            }
            j=j+1;
        }
        temp   =  arr[i];
        arr[i] =  arr[min];
        arr[min] =  temp;
        i=i+1;
    }
    arr
}

pub fn find_gretest_second_gretest(arr: &mut[i32]){
    let mut i =0;
    let mut first = 0;
    let mut second = 0;
    let mut arr_len = arr.len();
    first = 21111111111111;
    second =21111111111111;
    while i < arr_len {
        if arr[i] < first {
            second = first;
            first = arr[i];
        }
        else if arr[i] < second && arr[i] != first{
            second = arr[i];
        }


        i=i+1;
    }
    if second == 21111111111111{
        println!("There is no smallest element");
    }else{
        println!("First smallest element {}, and second smallest {}",first,second);
    }
}


/*
This is the quick sort
Worst Case Time Complexity : O(n2)
Best Case Time Complexity : O(n log n)
Average Time Complexity : O(n log n)
Space Complexity : O(n log n)
*/

pub fn quick_sort(arr: &mut[i32]) -> &mut[i32]{
    let mut start = 0;
    let mut arr_len = arr.len();
    if start < arr_len{
        let mut q = 0;
        q = partition(arr, start, arr_len);
        quick_sort(arr, start, q);
        quick_sort(arr,q+1,arr_len);
    }
}

fn partition(arrs: &mut[i32], p: &mut i32, arr_len_v: &mut i32) ->&mut i32{
    let mut i = 0 ;
    let mut pivot = 0 ;
    let mut j = 0 ;
    let mut temp = 0 ;

    pivot = arrs[p];
    i = p;
    j = arr_len_v;

    loop {
        while arrs[i] < pivot && arrs[i] !=pivot {
            i = i+1;
        }
        while arrs[j] > pivot && arrs[j] != pivot {
            j = j-1;
        }
        if i < j{
            temp = arrs[i];
            arrs[i] = arrs[j];
            arrs[j] = temp
        }
        else{
            j
        }
    }

}