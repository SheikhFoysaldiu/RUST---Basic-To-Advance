fn loop_fn()
{
    let mut i = 0;
    loop
    {
        i += 1;
        if i == 10
        {
            break;
        }
    }
    println!("i = {}", i);
}
fn while_fn()
{
    let mut i = 0;
    while i < 10
    {
        i += 1;
    }
    println!("i = {}", i);
}


fn main()
{
    loop_fn();
    while_fn();

}   