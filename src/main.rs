use ansi_term::Colour::*;

fn main() {
    let logo = concat!(
        "             /////////////             \n",
        "         /////////////////////         \n",
        "      ///////*767////////////////      \n",
        "    //////7676767676*//////////////    \n",
        "   /////76767//7676767//////////////   \n",
        "  /////767676///*76767///////////////  \n",
        " ///////767676///76767.///7676*/////// \n",
        "/////////767676//76767///767676////////\n",
        "//////////76767676767////76767/////////\n",
        "///////////76767676//////7676//////////\n",
        "////////////,7676,///////767///////////\n",
        "/////////////*7676///////76////////////\n",
        "///////////////7676////////////////////\n",
        " ///////////////7676///767//////////// \n",
        "  //////////////////////'////////////  \n",
        "   //////.7676767676767676767,//////   \n",
        "    /////767676767676767676767/////    \n",
        "      ///////////////////////////      \n",
        "         /////////////////////         \n",
        "             /////////////             \n",
    );

    let background = "/";
    let colored_letters = ["7", "6", "*", ",", ".", "'"];

    // The more spaces you add the fartest the text will be from the logo
    let spacing = "   ";

    let mut line_to_print: String = "".to_string();
    let mut additional_thing_to_print: String = spacing.to_string();
    let mut index = 0;
    logo.lines().for_each(|line| {
        line_to_print = line.to_string();

        // YOU CANNOT USE THE SAME COLOR FOR BACKGROUND AND COLORED_LETTERS
        // This is due to a bug in the library I use for printing colors

        for i in 0..colored_letters.len() {
            // We will replace every letter in the array "color_letters" with it's color
            // counterpant, to change the color of letters you can change the "White" in the next
            // line with the color you would like
            line_to_print = line_to_print.replace(colored_letters[i], &White.paint(colored_letters[i]).to_string());
        }

        additional_thing_to_print = spacing.to_string();

        match index {
            0 => { additional_thing_to_print += &"This will be on line 0/first line" }
            1 => { additional_thing_to_print += &"This will be on line 1/second line" }
            3 => { additional_thing_to_print += &"This will be on line 3/fourth line, we skipped one line" }
            4 => { additional_thing_to_print += &(Cyan.paint("This is how you add color ").to_string() + &"thank me later".to_string()) }
            _ => { additional_thing_to_print = spacing.to_string(); }
        }

        // Print the logo with the addition of the colored letters, and replace the contents of
        // "background" (in this case it's "/") with its color counterpart, change Cyan with the
        // color you would like for the "background"
        println!("{}", 
            line_to_print.replace(background, &Cyan.paint(background).to_string()) + &additional_thing_to_print
        );

        index += 1;
    });
}
