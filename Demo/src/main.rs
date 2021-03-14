// Installation >>> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX              Driver Code          XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
  
fn main(){

print();
variables();
mut_variables();
multiple_variables();
constant_varaibles();
functions(3.3,5.7);
path_module();
use_module();
Lists();
Tuples();
if_expression();
ranged_loop();
array_loop();
loops();
label_loop();
while_loop();
ownership();
immutable_references();
mutable_references();
structs();
tuple_struct();

}                         


// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX              Print Statement          XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

fn print() {
    println!("Hello, world!");
}


// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX             Variables          XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

// Declaring Variables - Variables in RUST are immutable by default (values can't change)


fn variables() {
    let age = 3;
    println!("{}",age);

}

// If you want to change value of a variable later, you'll have to add mut keyword

fn mut_variables() {
    let mut age = 7;
    println!("{}",age);
    age = 4;
    println!("{}",age);

}

// Assigning multiple variables at the same time

fn multiple_variables() {
    let (name,age) = ("Agha",21);
    println!("{}",age);
    println!("{}",name);
}

// constant variables - there are 3 requirements for constant variable

    //  1 - Snake Case for var name (All words capital, separated by underscore,eg: HELLO_WORLD)
    //  2 - Type Annotation is mandatory (Datatype)
    //  3 - The value musty be constant. It won't change.

fn constant_varaibles() {

    const WARP_FACTOR: f64 = 9.9;
    println!("{}",WARP_FACTOR);

}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX        Functions       XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

// when defining parameters, the type should be defined too, the arrow
// points to the datatype of whats returned or simply data type of output

fn functions(qty: f64, oz: f64) -> f64 {

    return qty*oz;
    // qty*oz (shorthand method without return keyword and semicolon, also called tail expression)
 
}



// XXXXXXXXXXXXXXXXXXXXXX       Module System - Calling one function from another file to current one       XXXXXXXXXXXXXXXXXXXX

//  all items in a library are public by default

fn path_module() {
    Demo::greet();          //This is path of greet from lib.rs

}

// Alternate and suggested method is to use, USE keyword, it is like import in python

use Demo::greet;
fn use_module() {
    greet();
} 

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX        Compound Types      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

            // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX       Lists      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX


fn Lists() {

    let array = [2.2, 3.4, 2.0, 5.1];
    println!("{:?}",array)

}

            // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX       Tuples      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

fn Tuples() {
    
    let tuple = (3,4,5);
    println!("{:?}",tuple)
            
}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX       Control Flow      XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX


    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  If Expressions  XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
 
fn if_expression() {

    let num = 4;

    if num==4{

        println!("four")
    }

    else if num==5 {
        println!("five")    }

    else{

        println!("None")   
    
    };
}

    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  For Loop  XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX


fn ranged_loop(){
 
for i in 0..50{                   // 0..=50 - to include 50 as well (inclusive and exclusive scenes)
        println!("{}",i);
    }

}

fn array_loop() {
    
    let array = [(1,2),(2,3)];
    for (x,y) in array.iter(){
        println!("{},{}",x,y);
    }
}

fn loops() {
    
    for i in [7,5,8].iter(){
    println!("{}",i);
    }
}

    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Labling a Loop  XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

fn label_loop(){

    'loop1: for i in 0..10 {
        for j in 1..5 {
            println!("{}", i);
            break 'loop1;
        }
    }
}


    // XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  While Loop  XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

fn while_loop() {

   while true {
        println!("1");
        break;
    }
}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Strings    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

//  Borrowed String Slice (&str) - can't be modified

// let msg = "abc".to_string(); 

//  contains length and capacity where length=capacity because immutable



// String

// let msg = String::from("abc"); 

//  contains length and capacity where capacity>length because mutable


// XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Ownership    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

// let s1 = String::from("abc");
// let s2 = s1;
// Now if i want to use s1, it won't let me, because setting s2=s1 moves value of s1 to s2
// and when we want to use s1, an error will arise due to it because s1 would be empty,
//  therefore to cater thi we have clone().


fn ownership() {
let s1 = String::from("abc");
let s2 = s1.clone();
println!("{},{}",s1,s2);
}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  References & Borrowing   XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX


// Immutable References

fn immutable_references(){

    let mut variable = 10;
    let reference_to_variable = &variable;
    println!("{},{}",variable,reference_to_variable)
}

//  This covers borrowing too, also, we can have multiple immutable references but only 1 mutable reference.

fn mutable_references(){

    let mut variable = 10;

    {  
        let reference_to_variable = &mut variable;
        *reference_to_variable += 1;
        println!("{}",reference_to_variable); 
    }
       
    println!("{}",variable);

    // Here, the println statement is actually borrowing the referce_to_variable, thats why the 
    // error arises because we can't define a mutable reference as well as borrow at the same time. 
    // TO avoid that, we enclose referencing part into a block with curly braces 

}

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  Structs    XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

//  Struct is same as classes

struct Color {

    red: u8,  // u8 => unassignemd 8 bit int i.e values from 0 to 255, just like that of rgb values
    blue: u8,
    green: u8,
    
}

fn structs() {

    let mut bg = Color { red: 50, green: 90, blue: 54 };
    bg.blue = 45;
    println!("{},{},{}", bg.red, bg.green, bg.blue);

}


tuple_struct(){

let red = Colors(45,87,09);
println!("{},{},{}", red.0, red.1, red.2);
        
    }

struct Colors(u8,u8,u8);
