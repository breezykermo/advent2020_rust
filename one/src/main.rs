// find 2 entries that sum to 2020, and then multiply those two numbers
use std::fs;

fn find_two(contents: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut res = None;
    for t in contents {
        let twin = sum - t;
        match contents.binary_search(&twin) {
            Ok(_) => {
                res = Some((*t, twin));
                break;
            }
            Err(_) => (),
        };
    }
    res
}

fn find_three(contents: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    let mut res = None;
    for t in contents {
        let twin = sum - t;
        match find_two(contents, twin) {
            Some((fst, snd)) => {
                res = Some((*t, fst, snd));
                break;
            }
            None => (),
        }
    }
    res
}

fn main() {
    let contents = fs::read_to_string("./input.txt".to_string())
        .expect("Something went wrong reading the file");

    let mut contents: Vec<_> = contents.split('\n').collect();
    contents.remove(contents.len() - 1);
    let mut contents: Vec<i32> = contents.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    contents.sort_unstable();
    assert_eq!(contents.len(), 200);

    let (one, two) = find_two(&contents, 2020).unwrap();
    println!("Answer to the first section: {}", one * two);
    let (one, two, three) = find_three(&contents, 2020).unwrap();
    println!("Answer to the second section: {}", one * two * three);
}
