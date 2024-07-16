//function that prints the length of each string in  a vector of vectors of strings
pub fn print_lengths(data: &Vec<Vec<String>>) -> () {
    for row in data {
        for item in row {
            println!("{} ", item.len());
        }
        println!("{} ", row.len());
    }
}
