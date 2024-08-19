use std::{i32, io};


fn do_addition()
{
    let (mut a, mut b) = ("".to_string(), "".to_string());

    println!("Please enter the first number \n");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to parse the number");
    a = a.trim_end().to_string();
    let num1 :i64 = a.parse::<i64>().unwrap();
    
    println!("Please select the 2nd number you want to add to \n");

    io::stdin()
        .read_line(&mut b)
        .expect("Failed to parse the 2nd number");

    b = b.trim_end().to_string();

    let num2 :i64 = a.parse::<i64>().unwrap();
    let result :i64 = num1 + num2;
    println!("The result is {}", result);

}


fn main() {
    let mut exit = false;
    let mut current_operation :String = "".to_string();

    while !exit
    {
        println!("Please enter an operation you want to perform : \n");

        io::stdin()
            .read_line(&mut current_operation)
            .expect("Failed to read the line");

        match current_operation.trim_end()
        {
            "add" =>   do_addition(),
            "exit" => exit = true,
            _ => println!("Unknown operation  {}, try again", current_operation),
            
        }
    }

    println!("Finished all operations ! exiting...");
}
