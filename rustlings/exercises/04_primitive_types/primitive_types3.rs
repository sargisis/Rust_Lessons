fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a  = [1,2,345,5,6,7,865,54,4,33,4,3,43,43,4,35,35,35,3,53,5,35,354,35,434,543,454,45,43,54,34,3,4,3,43,4,3,54,45,44,3,4,34,3,5,3,5,3,45,44,54,3,4,35,3,53,5,3,53,5,3,4,3,5,3,5,4,54,54,54,5,3433,33,4,5435443,2233,33,3,3,3,3,3,3,3,3,3,45,5,43,2,4,5,6,7,42,2,4,5,32,1,3,4,5,3,5,3,5,3,5,3,22,24,5,6,7];

    

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
