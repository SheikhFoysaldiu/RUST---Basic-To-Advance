enum Menu {
    Drink,
    Fries
}

fn main()
{
    let _paid = true;
    let item = Menu::Fries;
    let drink_type = "water";

    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "water"
            {
               true  
            }
            else 
            {
                false
            }
        },
        Menu::Fries => false,
        _ => true,
         
    };

    println!("{}",order_placed)
    



}