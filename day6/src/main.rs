use std::error::Error;
use std::fs;

const TESTCOMMS: &str = &"bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

const TESTCOMMS2: &str = &"mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

fn find_unique_window_pos(winsize: usize, comm: &str) -> i32 {
    let mut unique_pos: i32 = -1;

    for (i, _) in comm.chars().collect::<Vec<char>>().iter().enumerate() {
        if i > winsize - 1 {
            let window = &comm[i - winsize..i];
            let mut uniq: Vec<char> = Vec::new();
            let mut _isunique = false;

            for wc in window.chars() {
                if !uniq.contains(&wc) {
                    uniq.push(wc);
                }
            }

            if uniq.len() == winsize {
                _isunique = true;
                break;
            }
            unique_pos = (i + 1) as i32;
            // print!("{}:{}:[{}]:[{:?}] ", i, c, window, uniq);
        }
    }

    unique_pos
}
#[test]
fn test_example_comms() {
    let comms: Vec<&str> = TESTCOMMS.split("\n").collect();
    assert_eq!(5, find_unique_window_pos(4, &comms[0]));
    assert_eq!(6, find_unique_window_pos(4, &comms[1]));
    assert_eq!(10, find_unique_window_pos(4, &comms[2]));
    assert_eq!(11, find_unique_window_pos(4, &comms[3]));

    let comms2: Vec<&str> = TESTCOMMS2.split("\n").collect();
    assert_eq!(19, find_unique_window_pos(14, &comms2[0]));
    assert_eq!(23, find_unique_window_pos(14, &comms2[1]));
    assert_eq!(23, find_unique_window_pos(14, &comms2[2]));
    assert_eq!(29, find_unique_window_pos(14, &comms2[3]));
    assert_eq!(26, find_unique_window_pos(14, &comms2[4]));
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;
    let pos = find_unique_window_pos(4, contents.as_str());
    eprintln!("pos with window 4 = {:?}", pos);
    let pos = find_unique_window_pos(14, contents.as_str());
    eprintln!("pos with window 14 = {:?}", pos);
    Ok(())
}
