pub fn run(){
     //print to the console.
     println!("hello world");

     //basic formatting
     println!("{} is a {} programmer","harol","excellent");

     //positional args
     println!("{0} likes {1}","harol","pizza");

     //named args
     println!("today is {day} and {weather}", day="sunday",weather="sunny");

     //placeholder traits
     println!("BInary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

     //placeholder for debeg traits
     println!("{:?}",(12,true,"hello"));

     //basic math
     println!("10 + 10 = {}", 10 + 10);
}