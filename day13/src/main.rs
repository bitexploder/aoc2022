use serde_json::{json, Number, Result, Value};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;

fn compare_values(left: &Value, right: &Value) -> Ordering {
    let ordering: Ordering = match [left, right] {
        [Value::Number(_), Value::Number(_)] => {
            let (lnum, rnum) = (left.as_u64().unwrap(), right.as_u64().unwrap());

            if lnum < rnum {
                return Ordering::Less;
            } else if rnum < lnum {
                return Ordering::Greater;
            } else {
                Ordering::Equal
            }
        }

        [Value::Array(_), Value::Array(_)] => {
            let larr: &Vec<Value> = left.as_array().unwrap();
            let rarr: &Vec<Value> = right.as_array().unwrap();

            let mut ret = Ordering::Equal;

            for (i, v) in rarr.iter().enumerate() {
                let llen = larr[i..].len();

                // Condition: Left list out of items
                if llen == 0 {
                    ret = Ordering::Less;
                    break;
                }

                let lval = &larr[i];
                let rval = &v;

                match [lval, rval] {
                    [Value::Array(_), Value::Array(_)] => {
                        ret = compare_values(&lval, &rval);
                    }
                    [Value::Number(_), Value::Number(_)] => {
                        ret = compare_values(&lval, &rval);
                    }
                    [Value::Number(_), Value::Array(_)] => {
                        let arr = json!([lval]);
                        ret = compare_values(&arr, &rval);
                    }
                    [Value::Array(_), Value::Number(_)] => {
                        let arr = json!([rval]);
                        ret = compare_values(&lval, &arr);
                    }
                    _ => {
                        println!("hi ");
                    }
                }

                if ret != Ordering::Equal {
                    break;
                }
            }

            // Condition: Right list ran out
            if larr.len() > rarr.len() && ret == Ordering::Equal {
                ret = Ordering::Greater;
            }

            ret
        }

        _ => Ordering::Equal,
    };

    ordering
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/puzzle.txt")?;

    // Part 1
    let pairs: Vec<Vec<Value>> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split("\n")
                .map(|s| serde_json::from_str(s).unwrap())
                .collect::<Vec<Value>>()
        })
        .collect::<Vec<Vec<Value>>>();

    let mut correct_idx: Vec<usize> = Vec::new();
    for (i, pair) in pairs.iter().enumerate() {
        let correct = compare_values(&pair[0], &pair[1]);
        if correct == Ordering::Less {
            correct_idx.push(i + 1)
        }
    }
    let correct_sum = correct_idx.iter().sum::<usize>();
    println!("part 1 sum: {}", correct_sum);

    // Part 2
    let mut flat: Vec<Value> = pairs
        .iter()
        .map(|p| p.clone())
        .flatten()
        .collect::<Vec<Value>>();
    flat.push(json![[[2]]]);
    flat.push(json![[[6]]]);
    flat.sort_by(|left, right| compare_values(left, right));

    let mut i1 = 0;
    let mut i2 = 0;
    let d1 = json![[[2]]];
    let d2 = json![[[6]]];

    for (i, v) in flat.iter().enumerate() {
        if v == &d1 {
            i1 = i + 1;
        }
        if v == &d2 {
            i2 = i + 1;
        }
    }

    println!("divider index product: {}", i1 * i2);

    Ok(())
}
