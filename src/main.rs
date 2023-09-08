fn main() {
    println!("Hello, world!");

    // Variables
    let name = "kevin";
    let age:i8 = 22;


    // Types
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let rating_float2: f32 = 4.5;                // float type
    let is_growing_boolean = true;          // boolean type
    let is_fun: bool = true;                      // boolean type
    let icon_char = '♥';                    // unicode character type
    let emoji:char = '😁';                         //  unicode character type

    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}",icon_char);

    let result = 10;    // i32 by default
    let age:u32 = 20; // unsigned
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;

    println!("result value is {}",result);
    println!("sum is {  } and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);

    // immutable and mutable variables
    let fees = 25_000;
    // fees = "hola" // error

    let mut age:i32 = 22;
    age = 25;

    // constants
    const USER_LIMIT:i32 = 100;
    const PI: f32 = 3.14;

    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
    println!("pi value is {}",PI);

    // string objects
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("kevin");
    println!("length is {}",content_string.len());

}
