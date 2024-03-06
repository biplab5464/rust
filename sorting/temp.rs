//give a pivot (an index) make the index such the element smaller to them will be right of the element and bigger will be right to this element

fn main(){
    let mut arr = [12,45,85,72,78,57,78,6,76,2,64,34,63,5,91,81,56,5,32,65];

    let start = 0;
    let end = arr.len() - 1;
    let pivot = end;

    let mut wc = 0;

    //we are searching which are the small then pivot and bringing them into one part of the array
    for i in start..end{
        if arr[pivot] >= arr[i] {
            arr.swap(wc,i);
            wc +=1;
        }
        println!("iter {i}{:?}",arr);
    }
    arr.swap(pivot,wc);
    println!("arr -> {:?}",arr);
}