enum Direction
{
    Up,
    Down,
    Left,
    Right
}


fn wayofpath (path : Direction)
{
    match path {
        Direction::Up => println!("We are going up"),
        Direction::Down => println!("We are going down"),
        Direction::Left => println!("We are going left"),
        Direction::Right => println!("We are going right")
    }
}

fn main()
{
    wayofpath(Direction::Up);
    wayofpath(Direction::Down);
    wayofpath(Direction::Left);
    wayofpath(Direction::Right);

}