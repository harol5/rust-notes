fn main() {
    println!("Hello, world!");
    
    //i32 defines the type of varible (integer 32bits?)
    let x: i32 = 5;
    let _y: i32;
    
    //this chackes if arguments are equal;
    assert_eq!(x, 5);
    //this how you print in console
    println!("done");
    
    //the "mut" keyword mean the variable is mutable.
    let mut a: i32 = 1;
    
    //augm operator.
    a += 2;
    assert_eq!(a, 3);
    println!("done");
    
    other();
    print_name();
    practicing_variables();
}

fn other(){
    //&str defines the type of varible (string)
    let x: &str = "hello";
    
    //the curly braces serve as a place holder for arguments passed.
    println!("{}, wolrd",x);
}
fn print_name(){
    let name: &str = "harol";
    let age: i32 = 28;
    
    assert_eq!(age, 28);
    println!("hello {}, you are {}",name,age);
    
    //destructuring.
    let (car_one,car_two) = ("bmw","honda");
    println!("{} and {}",car_one,car_two);
    
    
    let (x,y);
    (x,..) = (3,4);
    [..,y] = [1,2];
    
    assert_eq!([x,y], [3,2]);
    println!("doneee!!!");
}

fn practicing_variables(){
    //32 means the type of number by # of bits;
    //the letter "u" means it can only hold positives numbers,
    //the letter "y" means it can hold negative number as well.
    //these rules apply to the other types of numbers (8,16,32,64,128);
    let numero: u32 = 4000000000;
    let otro: i32 = 2000000000;
    println!("{} and {}",numero,otro);
}