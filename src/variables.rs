pub fn types(){
     //the "mut" keyword mean the variable is mutable.
     let mut a: i32 = 1;

     //const
     const ID: i32 = 122;

     //assign multiple vars
     let (my_name, my_age) = ("gloria",50);

     println!("{:?}",(a,ID,my_name,my_age));
}

pub fn primitive(){
     //32 means the type of number by # of bits;
     //the letter "u" means it can only hold positives numbers,
     //the letter "y" means it can hold negative number as well.
     //these rules apply to the other types of numbers (8,16,32,64,128);
     let x: u32 = 4000000000;
     let y: i32 = 2000000000;

     //float numbers
     let pi: f64 = 3.1416;

     // find max size
     println!("max i32: {}", std::i32::MAX);

     //boolean
     let is_active: bool = true;

     //get boolean from exp.
     let is_greater = 10 > 5;

     //chars
     let letter: char = 'a';

     //any unicode char
     let face = '\u{1f600}';

     println!("{:?}",(x,y,pi,is_active,is_greater,letter,face))
}

pub fn strings(){
     //there a 2 types of strings: primitive(fixed size) and heap allocated string;

     //primitive.
     let name: &str = "gloria";

     //heap, growable.
     let mut last_name = String::from("perez ");

     //get length.
     println!("length (primitive): {}", name.len());
     println!("length (heap): {}", last_name.len());

     //add a char to heap string.
     last_name.push('A');
     
     //add a string to heap string
     last_name.push_str(" car");
     
     //capacity in bytes.
     last_name.capacity();

     //check if empty
     last_name.is_empty();

     //contains
     last_name.contains("perez");

     //replace (return a new string).
     let new_last_name = last_name.replace("perez", "rojas");
     println!("{}",new_last_name);

     //loop through string by whitespace
     for word in last_name.split_whitespace(){
          println!("{}",word);
     }

     //create string with capacity.
     let mut country: String = String::with_capacity(10);     

}

pub fn tuples(){
     //group together values of different types.
     //max elements: 12.

     let person: (&str, &str, i8) = ("harol", "rojas",28);
     println!("{name} {last_name} is {age}",name=person.0,last_name=person.1,age=person.2);
}