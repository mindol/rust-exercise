fn main() {
    let ordinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming",
        "Eight maids-a-milking",
        "Nine ladies dancing",
        "Ten lords-a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let first_present = "A partridge in a pear tree";

    for i in 0..12 {
        println!("On the {} day of Christmas,", ordinal_numbers[i]);
        println!("My true love gave to me,");

        match i {
            0 => println!("{}.", first_present),
            _ => {
                for j in (1..=i).rev() {
                    println!("{},", presents[j]);
                }
                println!("{}.", presents[0]);
            }
        }
        if i != 11 {
            println!("\n");
        }
    }
}
