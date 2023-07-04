fn sub(x: i32, y: i32)-> i32 {
    x - y
}

fn add(x: i32, y: i32)-> i32 {
    x + y
}

fn mul(x: i32, y: i32)-> i32 {
    x * y
}

fn main()
{
    let x = sub(10, 5);
    println!("substract = {}", x);

    let y = add(10, 5);
    println!("Summation = {}", y);

    let z = mul(10, 5);
    println!("Multiplication = {}", z);

}