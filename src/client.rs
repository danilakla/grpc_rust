use tonic::transport::Channel;
use greet::greeter_client::GreeterClient;
use greet::{Parm1Request, Parm2Request};

pub mod greet {
    tonic::include_proto!("greet");
}
#[allow(dead_code)]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    loop {
        println!("Choose an operation: Add, Sub, Mul, Div, Pow2");
        let mut operation = String::new();
        std::io::stdin().read_line(&mut operation)?;
        let operation = operation.trim().to_lowercase();

        match operation.as_str() {
            "add" | "sub" | "mul" | "div" => {
                let (x, y) = get_two_numbers();
                let request = tonic::Request::new(Parm2Request { x, y });

                let response = match operation.as_str() {
                    "add" => client.add(request).await?.into_inner(),
                    "sub" => client.sub(request).await?.into_inner(),
                    "mul" => client.mul(request).await?.into_inner(),
                    "div" => client.div(request).await?.into_inner(),
                    _ => unreachable!(),
                };
                println!("Result: x = {}, y = {}, z = {}", response.x, response.y, response.z);
            }
            "pow2" => {
                let x = get_one_number();
                let request = tonic::Request::new(Parm1Request { x });

                let response = client.pow2(request).await?.into_inner();
                println!("Result: x = {}, z = {}", response.x, response.z);
            }
            _ => {
                println!("Invalid operation. Try again.");
            }
        }
    }
}

fn get_two_numbers() -> (i32, i32) {
    println!("Enter two integers (x and y):");

    let x = read_number("x");
    let y = read_number("y");

    (x, y)
}

fn get_one_number() -> i32 {
    println!("Enter an integer (x):");
    read_number("x")
}

fn read_number(prompt: &str) -> i32 {
    loop {
        println!("{}: ", prompt);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse() {
            return num;
        }
        println!("Invalid input. Please enter a valid integer.");
    }
}
