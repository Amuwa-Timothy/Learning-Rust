fn main () {
    let gifts = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
    ];

    let day_names = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for day_number in 1..13{
        println!("On the {} day of Christmas, my true love sent to me:", day_names[day_number - 1]);

        for gift_index in (0..day_number).rev(){
            println! ("{}", gifts[gift_index]);

        }

    }
}

    

