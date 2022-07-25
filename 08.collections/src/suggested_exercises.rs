use rand::Rng; // 0.6.5

pub fn exercises(){
    let mut rng = rand::thread_rng();
    let mut vals: Vec<u32> = (0..50).map(|_| rng.gen_range(0..20)).collect();
    
    vals.sort_unstable();
    println!("{:?}", vals);

    let median = vals.get(25).unwrap();
    println!("median value is: {}", median);

    let initial = String::from("this is a normal string.");
    for s in initial.split_ascii_whitespace(){
        println!("{}", convert_to_pig_latin(s))
    }

    
}

pub fn convert_to_pig_latin(word: &str) -> String{

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut starts_with_vowel = false;
    if vowels.iter().any(|&v| word.starts_with(v)) {
        starts_with_vowel = true;
    }

    let mut res = String::from(word);
    if starts_with_vowel{
        res += "-hay"
    }else{
        let first = res.chars().next().unwrap().to_string();
        res += &String::from("-".to_string() + &first + "ay");
        res.remove(0);
    }

    res
}