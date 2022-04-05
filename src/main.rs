fn main() {
    let a = 15;
    let b = 16;
    println!("Hello world! {} {}", a, b);

    const UNSIGNED: u8 = 10;
    println!("unsign {}", UNSIGNED);

    const SIGN: i8 = -45;
    println!("sign {}", SIGN);

    let float: f32 = 1.8;
    println!("sign {}", float);

    let letter = "c";
    println!("letter {}", letter);

    let is_true = false;
    println!("is true {}", is_true);

    // Arrays

    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [i8; 5] = [-100; 5];

    println!("index {} , length {}", arr[0], other_arr.len());
    println!("{:?}", other_arr);

    //Tuples
    let tuple: (u8, bool, f32) = (5, true, 2.10);
    let tuple_2 = (3, 5);

    println!("first {} , second {} , third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple_2);

    let (a, b, c) = tuple;
    println!("first {} , second {} , third {}", a, b, c);

    //Function

    {
        println!("{}", is_even(1))
    };

    pub fn is_even(num: u8) -> bool {
        let digit: u8 = num % 2;
        digit == 0
    }

    //Mutability

    let mut num = 5;
    println!("{}", num);
    num = 3;
    println!("{}", num);

    //Arrays + slice
    {
        let arr = [0, 1, 2, 3, 4]; //length
        let slice = &arr[1..3];
        borrowing_sllice(arr, slice)
    } //[1,2] wec dont know the length

    fn borrowing_sllice(arr: [u8; 5], slice: &[u8]) {
        println!("{:?}", arr);
        println!("{:?}", slice);
        println!("length {}", slice.len());
        println!("{} {}", slice[0], slice[1]);
    }

    //Strings
    let str = "hello world";
    println!("{}", str);
    let mut string: String = String::from("hello world");
    let _slices = &string[..6];
    // slices.len();

    string.push('1');
    string.push_str(" Huzail");
    string = string.replace("hello", "bye");
    println!("{}", string);

    //if statememnt
    let n = 3;
    if n > 0 {
        println!("greater than 0");
    } else if n < 3 {
        println!("less than 3");
    } else {
        println!("is 0")
    }

    //For loop
    for i in 0..6 {
        println!("{}", i)
    }

    //while loop
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exit");
            break;
        }
    }

    //match statement or switch statement

let q = 5;
match q{
    0 => println!("0"),
    1|2 => println!("1,2"),
    3..=4 => println!("3,4"),
    _=> println!("default"),

}

//struct 
let name = String::from("Bird");
let bird = Bird{name , attack:5};
bird.print_name();
println!("{} {}" , bird.can_fly(),bird.is_animal());

struct Bird {
    name: String,
    attack : u64,
}

impl Bird {
    fn print_name(&self){
        println!("{} {} " , self.name , self.attack )
    }
}

//Traits

impl Animal for Bird {
    fn can_fly(&self)->bool{
        true
    }

    fn is_animal(&self)->bool{
        false
    }
}


trait Animal{
    fn can_fly(&self)->bool;
    fn is_animal(&self)->bool{
    true
    }
}


//Enums

{let a: MyEnum = MyEnum::A;
let b: MyEnum = MyEnum::B(5);
let c : MyEnum = MyEnum::C { x: 10, y: 20 };
println!("{:?}" , a);
println!("{:?}" , b);
println!("{:?}" , c);

// if  let MyEnum::B(val) = b {
//     println!("{}" , val);
// }
// if let MyEnum::C(x,y) = c{
//     println!("{} {}" , x, y);
// }

}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x:i32 , y:i32},
}

//vector
 let mut vec:Vec<i64> = vec![1,2,3,4,5];
 vec.len();
 vec[0];
 vec.push(6);
 vec.remove(0);
 println!("{:?}" , vec);
 
//Hashmap
 use std :: collections::HashMap;

 {
     let mut map = HashMap::new();

     map.insert(0, "Hi");
     map.insert(1, "Hi2");
     println!("{:?}" , map);


     match  map.get(&0) {
         Some(str1)=>println!("{}" , str1),
         _=> println!("Dooes not exist in map")
     }

     match map.get(&2) {
         Some(str)=>println!("{}" , str),
         _=> println!("Dooes not exist in map"),
     }
     map.remove(&0);
     println!("{:?}" , map)
 }

 //options

 //None , to indicate failure or lack of value , and
 //some (value) , a tuple struct that wraps a value with type T

 fn divide(dividend: i32 , divisor: i32)-> Option<i32>{
     if dividend % divisor != 0{
         None
     }else {
         Some(dividend/divisor)
     }
}

{
    let divide1: Option<i32> = divide (4,2);
    let divide2: Option<i32> = divide (2,3);

    println!("{:?} unwraps to {}" , divide1 , divide1.unwrap());
    println!("{:?} unwraps to {}" , divide2 , divide2.unwrap());
    
}



}
