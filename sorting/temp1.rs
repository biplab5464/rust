//merged two sorted array

fn main(){
    let arr1 =  [2,4,8,10,12,14,16,18,20,22];   //[1,3,5,9,13,17,19,29,31,41];
    let arr2 =  [1,3,5,9,13,17,19,29,31,41];    //[2,4,8,10,12,14,16,18,20,22];

    let arr1_len = arr1.len();
    let arr2_len = arr2.len();

    let mut x = 0;
    let mut y = 0;

    let arr_len = arr1_len + arr2_len; 

    let mut merged_arr : Vec<usize> = Vec::new();

    dbg!(arr_len);

    while x < arr1_len && y < arr2_len {
        if arr1[x] <= arr2[y]{
            merged_arr.push(arr1[x]);
            x +=1;
        }else{
            merged_arr.push(arr2[y]);
            y +=1;
        }
        println!("---> {x} {y}");
    }

    if x < arr1_len{
        merged_arr.extend_from_slice(&arr1[x..]);
    }
    if y < arr2_len{
        merged_arr.extend_from_slice(&arr2[y..])
    }

    println!("arr => {:?}",merged_arr);
}