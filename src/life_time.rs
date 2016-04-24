//This will illustate about the lfetime and ownership

pub fn call_me(){
    // not_access_resource_mutable_borrow();
    //borrower_can_move_mut_borrow();
    //borrow_formula();
    struct_named_borrow()
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

//borrower scope.
// note here :
/*
First note that ,
& = borrow
There men't to be the alwys and it's necessory to the scope of resource is alwys greter than the scope of borrow scope or it's union of all borrowr's scope.
*/

/*
Let's See an Example */

fn borrow_formula(){
    let mut a = Foo {f: Box::new(0)};
    let y: &Foo;

    if false{
        println!("i'm in");
        //borrow
        let x = &a;
        //Shsre the borrow with  new borrower y, hence extend the borrow scope
       y = x;
    }
    //

    /*a.f = Box::new(2);
    println!("{:?}", a.f);*/
}

/*
Named borrow scope :
    When there are multiple & pointers as input , we need to specify their relationship using "named lifetime" as defined in Lifetime guide. But for now , lets
    just call them named borrow scopes.

        fn max(x: &Foo, y: &Foo) -> &Foo

    This code not going to work.
   without specifing the relationship between borrowers, i.e. which borrowers are grouped in which borrow scope. the following implementation is valid.
*/

fn max<'a>(x: &'a Foo, y: &'a Foo) -> &'a Foo{
    if x.f > y.f { x } else { y }
}

/*
Struct as a borrower :
    in addition to fuction and closures, a struct can also borrow multiple resources by storing multiple references in its fields. we'll see some example below and how the
    borrow formula applies. Let's use this Link stuct to store a reference (an immutable borrow)
*/

struct Link<'a>{
    link: &'a Foo,
}

fn struct_named_borrow(){
    let a = Foo {f: Box::new(4)};
    let mut x = Link {link: &a};
    if false{
        let b = Foo {f: Box::new(5)};
        //error: `b` does not live long enough
        x.link = &b;
    }

}