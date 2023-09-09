use std::arch::x86_64::__get_cpuid_max;
use std::fmt;

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


    // conditionals
    if age > 18{
        println!("eres mayor de edad")
    } else if age < 18 {
        println!("eres menor de edad")
    } else {
        println!("tienes 18 años")
    }

    // match statement
    let status_code = 200;
    match status_code {
        200 => println!("ok"),
        400 => println!("bad request"),
        _ => println!("unknown"),
    };

    // loops
     for num in  1..11{ // 11 is not inclusive
         if num%2==0 {
             continue;
         }
         println!("x is {}",num);
     }

    let mut y = 0;
    while y < 10 {
        y+=1;
        println!("inside loops 'Y' value is {}", y);
    };

    // while true
    let mut x = 0;
    loop {
        x+=1;
        println!("x={}",x);

        if x==15 {
            break;
        }
    }

    // functions
    fn hello(name: &str) {
        println!("¡Hello {}!", name)
    }

    hello("kevin");

    fn get_pi_value() -> f64 {
        22.0/7.0 // implicit return
        // return 22.0/7.0
    }

    let pi_value = get_pi_value();
    println!("PI value is {}", pi_value);


    // structures
    let tuple_name1 = ("kevin", 22);
    let tuple_name2:(&str, i32) = ("kevin", 22);
    println!("{:?}", tuple_name1);
    println!("string is {:?}",tuple_name1.0);
    println!("integer is {:?}",tuple_name1.1);

    let my_array = [1, 2, 3, 4, 5];
    let my_array2:[i32;5] = [6, 7, 8, 9, 10];
    let my_array3:[i32;10] = [1;10];

    for index in 0..my_array.len() {
        println!("index is: {} & value is : {}",index,my_array[index]);
    }
    for value in my_array2.iter() {
        println!("value is :{}",value);
    }
    // mutable array
    let mut arr:[i32;4] = [10,20,30,40];
    arr[1] = 0;
    println!("{:?}",arr);

    // slice
    let mut data = [10, 20, 30, 40, 50];
    let my_slice = &data[1..4];
    println!("{:?}",my_slice);
    println!("{:?}", data);

     let my_struct: Employee = Employee {
         name:String::from("kevin dueñas"),
         company:String::from("Apple"),
         age:22
     };
    let my_struct2 = Employee {
        name:String::from("alberto del rio"),
        company:String::from("Amazon"),
        age:50,
    };
    fn who_is_elder (emp1:Employee,emp2:Employee) -> Employee {
        if emp1.age>emp2.age {
            return emp1;
        }
        return emp2;

    }
    let elder = who_is_elder(my_struct, my_struct2);
    println!("Name is :{} company is {} age is {}",elder.name,elder.company,elder.age);
    println!("Age is {}", elder.show_age());

    let p1 = Point::get_instance(10,20);
    p1.display();

    // enums
    let person1 = Person {
        name:String::from("juanito"),
        gender:GenderCategory::Male,
    };
    let person2 = Person {
        name:String::from("dani"),
        gender:GenderCategory::Female,
    };

    println!("{:?}",person1);
    println!("{:?}",person2);

    fn print_size(car:CarType) {
        match car {
            CarType::Hatch => println!("Small sized car"),
            CarType::Sedan => println!("medium sized car"),
            CarType::SUV => println!("Large sized Sports Utility car"),
        }
    }
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);




    // unpacking
    let kevin = (22, "kevin", true);
    let (age, name, married) = kevin;
    println!("{}, {}, {}", age, name, married);

    let Point {x, y} = p1;
    println!("x is {} y is {}", x, y);

    let Person {name, gender} = person1;
    println!("name is {}, gender is {:?}", name, gender);

    let arr: [i32;10] = [1;10];
    let [first, second, rest @..] = arr;
    println!("Primer elemento: {}", first);
    println!("Segundo elemento: {}", second);
    println!("Resto del array: {:?}", rest);


}

enum CarType {
    Hatch,
    Sedan,
    SUV
}

#[derive(Debug)]
enum GenderCategory {
    Male, Female
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: GenderCategory
}

struct Employee {
    name: String,
    company: String,
    age: u32
}

impl Employee {
    fn show_age(&self) -> u32 {
        self.age
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_instance(x: i32, y: i32) -> Point {
        Point{x, y}
    }
    fn display(&self){
        println!("x={} y={}",self.x,self.y);
    }
}