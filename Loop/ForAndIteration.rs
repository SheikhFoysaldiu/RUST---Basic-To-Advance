// into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.


// iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
fn ForAndIter()
{
    
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
// output:
// Hello Bob
// Hello Frank
// There is a rustacean among us!
// names: ["Bob", "Frank", "Ferris"]
}

// into_iter - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

fn ForAndIntoIter()
{
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
// output:
// Hello Bob
// Hello Frank
// There is a rustacean among us!
// names: ["Bob", "Frank", "Ferris"]
}

// iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
fn ForAndIterMut()
{
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    
    println!("names: {:?}", names);
// output:
// names: ["Hello", "Hello", "There is a rustacean among us!"]
}




fn main() {
    ForAndIter();
    ForAndIntoIter();
    ForAndIterMut();
}


