fn main() {
    let lyrics = ["Twelve Drummers Drumming,", "Eleven Pipers Piping,", "Ten Lords a Leaping,", "Nine Ladies Dancing,", "Eight Maids a Milking,", "Seven Swans a Swimming,", "Six Geese a Laying,", "Five Golden Rings,", "Four Calling Birds,", "Three French Hens,", "Two Turtle Doves,", "...and a Partridge in a Pear Tree!"];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut i = 0;
    let mut n = 11;

    while i <= n {
        println!("On the {} day of Christmas my true love sent to me...", days[i]);

        if i == 0 {
            println!("...a Partridge in a Pear Tree!");
            i = i + 1;
            continue;
        }

        let mut x = n - i;

        while x <= n {
            println!("{}", lyrics[x]); // got to loop through right array elements
            x = x + 1;
        }

        i = i + 1;
    }
}
