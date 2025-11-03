fn main () {

    let sentence: String = String::from("my name is Vishal");
    let first_word: String = get_first_word(sentence);
    print!("First word is {}", first_word);

}

fn get_first_word(sentence: String) -> String {

    let mut ans: String = String::from("");

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }  
    }

    return ans;
}
