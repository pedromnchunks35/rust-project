fn main() {
    //? String literal (static by default)
    let company: &str = "Tutorials point";
    //? String literal with explicit static
    let company: &'static str = "Tut";

    //? String object
    //? Empty string
    let empty_string = String::new();
    //? Normal String
    let stringas = String::form("Hello world");
    //? imp methods
    //? converts into string object
    let name1="Hello Tuts".to_string();
    name1.replace("Hello","kkk");
    //? converts to string
    name1.as_str();
    //? adds a char
    name1.push('a');
    //? adds a string
    name1.push_str("Hello");
    name1.len();
    //? removes spaces
    name1.trim();
    //? For having a iterator over the strings (white space)
    for token in name1.split_whitespace(){
        //....
    }
    //? Split over commaas
    for token in name1.split(){
        //....
    }
}
