//This will illustate about the lfetime and ownership

pub fn call_me(){
   // not_access_resource_mutable_borrow();
    borrower_can_move_mut_borrow();
}

struct Foo{
    f: Box<i32>,
}

//Owner can not access resource during a mutable borrow
fn not_access_resource_mutable_borrow(){
    let mut a = Foo{f: Box::new(0)};        //==> Owner
    //mutable borrow
    //let x = &mut a;                         //==> borrower
    //error: cannot borrow `a.f` as immutable because `a` is also borrowed as mutable [E0502]
    //println!("{}",a.f);                     //==> Owner can't access during mutable borrow

    // For This you have to use scope or ther lifetime of the variable.

    //This will work.
    {                       //--> scope starts
        let x = &mut a;     //==> borrower
    }                       //--> scope end's

    println!("{}",a.f);
}


//Borrower can move the mutable borrow to a new borrower
fn borrower_can_move_mut_borrow(){
    let mut a = Foo{f: Box::new(0)};        //==> Owner

    let x = &mut a;                         //mutable borrow

    let y = x;                               //transfering mutable borrow of the mutable borrow.

    //println!("{}",x.f);                      //This give an error cause borrow moved. let's test it
    //error: use of moved value: `x.f` [E0382]

    println!("{}", y.f);                    // ==> This is okay

    //after move, the orignal borrower (x) can no longer access the borrowed resource.
}