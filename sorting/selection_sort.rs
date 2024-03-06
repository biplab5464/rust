//pratice of selection sor
fn main(){
    let mut arr = [12,45,85,78,78,457,78,6,756,46,648,64,658,5,91,81,56,5,32,80];
    let len = arr.len() -1;
    quick_sort(&mut arr,0,len);

    println!("arr -> {:?}",arr);
}


fn quick_sort(arr : &mut [i32], start : usize, end : usize){
    let pivot = end;
    let mut cp = start;
    if start < end {
        for index in start..end{
            if arr[pivot] >= arr[index]{
                arr.swap(index,cp);
                cp +=1;
            }
        }
        arr.swap(pivot,cp);

        quick_sort(arr, start, pivot);
        quick_sort(arr, pivot +1 ,end);
    }
}