 fn abomination(i:usize) -> String {
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];
    let tuple1 = ("On the ","{currentDay}"," day of Christmas, my true love sent to me");
    let mut return_string: String = String::new();
    return_string.push_str(tuple1.0);
    return_string.push_str(days[i]);
    return_string.push_str(tuple1.2);
    return return_string;
}

fn main() {
    let lyrics: [&str;12] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
    ];
     for (index, _element) in lyrics.iter().enumerate(){
        let mut final_lyric: Vec<&str> = Vec::new();

        //filling up the vector
        for i in (0..index+1).rev(){
            //println!("{i}");
            final_lyric.push(lyrics[i]);
        }
        //filling up the vector
        println!("{}", abomination(index));//prints "on the xth day of christmas", the constant in the song

        for (index, _element1) in final_lyric.iter().enumerate(){
            println!("{}",final_lyric[index]);
        }
        println!("\n");
    }  
}
