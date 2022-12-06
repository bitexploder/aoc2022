use bit_set::BitSet;
use itertools::Itertools;
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

// Functional style
fn find_unique_window_posv2(winsize: usize, comm: &str) -> usize {
    let windows: Vec<usize> = comm
        .chars()
        .collect::<Vec<char>>()
        .windows(winsize)
        .map(|win| win.iter().unique().collect::<String>().len()) // only need len
        .take_while(|l| *l != winsize)
        .collect::<Vec<usize>>();

    windows.len() + winsize
}

fn find_unique_window_posv3(winsize: usize, comm: &str) -> i32 {
    let mut unique_pos: i32 = -1;
    let mut bits = BitSet::new();
    let cbytes = comm.as_bytes();

    for (i, _) in cbytes.iter().enumerate() {
        if i > winsize - 1 {
            let window = &cbytes[i - winsize..i];

            for c in window {
                bits.insert((c - 97) as usize);
            }

            if bits.len() == winsize {
                unique_pos = (i + 1) as i32;
                break;
            }

            //print!("{}:[{:?}]:[{:?}] ", i, window, cbytes);
        }
        bits.clear();
    }

    unique_pos
}

#[test]
fn test_bitvecs() {
    let astr: Vec<u8> = vec![10, 11, 12, 10];
    let winsize: usize = 14;
    let mut bits = BitSet::new();
    eprintln!("astr = {:?}", astr);

    // let mut bits = bitvec![0;32];
    // bits.set(10, true);
    for b in astr {
        bits.insert(b as usize);
    }

    eprintln!("bits = {:?}, len = {:?}", bits, bits.len());
}

#[test]
fn test_example_comms3() {
    let comms: Vec<&str> = TESTCOMMS.split("\n").collect();
    assert_eq!(5, find_unique_window_posv3(4, &comms[0]));
    assert_eq!(6, find_unique_window_posv3(4, &comms[1]));
    assert_eq!(10, find_unique_window_posv3(4, &comms[2]));
    assert_eq!(11, find_unique_window_posv3(4, &comms[3]));
    //
    let comms2: Vec<&str> = TESTCOMMS2.split("\n").collect();
    assert_eq!(19, find_unique_window_posv3(14, &comms2[0]));
    assert_eq!(23, find_unique_window_posv3(14, &comms2[1]));
    assert_eq!(23, find_unique_window_posv3(14, &comms2[2]));
    assert_eq!(29, find_unique_window_posv3(14, &comms2[3]));
    assert_eq!(26, find_unique_window_posv3(14, &comms2[4]));
}

#[test]
fn test_example_comms2() {
    let comms: Vec<&str> = TESTCOMMS.split("\n").collect();
    // find_unique_window_posv2(4, &comms[0]);
    assert_eq!(5, find_unique_window_posv2(4, &comms[0]));
    assert_eq!(6, find_unique_window_posv2(4, &comms[1]));
    assert_eq!(10, find_unique_window_posv2(4, &comms[2]));
    assert_eq!(11, find_unique_window_posv2(4, &comms[3]));

    let comms2: Vec<&str> = TESTCOMMS2.split("\n").collect();
    assert_eq!(19, find_unique_window_posv2(14, &comms2[0]));
    assert_eq!(23, find_unique_window_posv2(14, &comms2[1]));
    assert_eq!(23, find_unique_window_posv2(14, &comms2[2]));
    assert_eq!(29, find_unique_window_posv2(14, &comms2[3]));
    assert_eq!(26, find_unique_window_posv2(14, &comms2[4]));
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
    let pos = find_unique_window_posv3(4, contents.as_str());
    eprintln!("pos with window 4 = {:?}", pos);
    let pos = find_unique_window_posv3(14, contents.as_str());
    eprintln!("pos with window 14 = {:?}", pos);
    Ok(())
}
