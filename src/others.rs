pub fn utils(){
     //this checks if arguments are equal;
    assert_eq!(x, 5);
}

fn print_name(){
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
     

     //remenber type of varibale in order to make operation like this.
     let mut _z: i32 = 6;
     _z = y;
     println!("{} and {}",_z,y);

     //casting variable.
     let _v: u16 = 38_u8 as u16;
}
