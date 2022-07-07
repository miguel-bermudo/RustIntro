use std::string;

pub fn first_word_slice(s: &str) -> &str{
    // returns the first word of a string, function without slices.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}

pub fn other_slices(s:&[i32]){
    assert_eq!(s, &[2, 3]);
    println!("{}",s[0])
}