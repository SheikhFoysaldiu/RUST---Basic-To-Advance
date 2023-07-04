
fn evenOdd(x: i32)
{
    if x % 2 == 0
    {
        println!("{} is even", x);
    }
    else
    {
        println!("{} is odd", x);
    }
}


fn primeNumber(x: i32)
{
    let mut i = 2;
    while i < x
    {
        if x % i == 0
        {
            println!("{} is not prime number", x);
            break;
        }
        i += 1;
    }
    if i == x
    {
        println!("{} is prime number", x);
    }
}

fn main()
{
    evenOdd(10);
    evenOdd(11);
    primeNumber(10);
    primeNumber(11);

}
