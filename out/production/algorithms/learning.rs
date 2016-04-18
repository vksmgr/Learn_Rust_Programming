//This module is for learning the rust programming in which we are going to learn basic concepts of rust programming language.
#[derive(Debug,Clone, Copy)]
pub enum Name{
    Vikas,
    Shyam,
    Ram,
}

pub fn Hello(){
    let vikas = Name::Vikas;
    let shyam = Name::Shyam;
    let ram = Name::Ram;
    print_name(vikas);
}
pub fn print_name(name: Name){
    //use Name::*;
        match name {
        Name::Vikas => hello_vikas(),
        Name::Shyam => hello_Shyam(),
        Name::Ram => hello_ram(),
    };
}
fn hello_vikas(){
    println!("hello Vikas");
}

fn hello_Shyam(){
    println!("hello Shyam");
}
fn hello_ram(){
    println!("hello Ram");
}