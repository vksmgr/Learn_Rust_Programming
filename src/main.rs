use std::io;
//mod sorting;
//mod learning;
mod life_time;
fn main() {

    //This is just Learning module
   // learning::Hello();
    life_time::call_me();
   /* println!("This is introduction to algorithms");*/
   /* let s = getInput();
    println!(" you entered the String : {:?} ",s);
    let int = readInt();
    println!("This is just and integer number: {}",int);
*/
    //=============================================Array=================
   /* println!("Arrays .. +++");
    let mut array: [i32;3] = [0;3];*/
    //printArray(&array);
    //println!("Now i wan't to get element of array from user");

   /* println!(" :::: ALGORITHMS :::: ");
    println!("Enter element to be sorted : ");
    for x in 0..array.len(){
        array[x] = readInt();
    }*/
   /* println!("Sorted Elements : ");

    //====================================INSERTION SORT =================
    println!(" 1] INSERTION SORT : ");
    printArray(&sorting::insertion_sort(&mut array));
    //printArray(&sorting::bubble_sort(&mut array));

    //====================================Selection SORT =================
    println!(" 1] INSERTION SORT : ");
    printArray(&sorting::selection_sort(&mut array));
*/
   // sorting::find_gretest_second_gretest(&mut array);

   // printArray(&sorting::quick_sort(&mut array));



}


//This function will print the value of array of type i32
fn printArray(arr: &[i32]){
    for x in 0..arr.len(){
        println!("{}",arr[x]);
    }
}
//This will gate string from user
fn getInput() ->String{
    let mut input_text = String::new();
    io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");
    input_text
}

//This will get number i32 type
fn readInt() -> i32{
    let s = getInput();
    let trimmed = s.trim();
    let intNumber =  match  trimmed.parse::<i32>() {
                            Ok(i) => i,
                            Err(..) => -1
                            };
    intNumber
}

