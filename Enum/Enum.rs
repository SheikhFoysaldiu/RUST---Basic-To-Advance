// The enum keyword allows the creation of a type which may be one of a few different variants. 
// Any variant which is valid as a struct is also valid in an enum.

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