use serde_json::Value;
use aoclib::{read_as_string, read_lines};

pub fn run() {
    let input = read_as_string("input/2015/day12.txt");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

fn part1(input: String) -> i64 {
    let v = serde_json::from_str(input.as_str()).expect("Unexpected to parse the input json");
    sum_json(&v)
}

fn part2(input: String) -> usize {
    1
}

fn sum_json(value: &Value) -> i64 {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(|a| sum_json(a)).sum(),
        Value::Object(obj) => obj.values().map(|v| sum_json(v)).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_sum_array() {
        let v = serde_json::from_str("[1,2,3]").expect("failed to parse json");
        assert_eq!(sum_json(&v), 6);
    }

    #[test]
    fn can_sub_object() {
        let v = serde_json::from_str("{\"a\":{\"b\":4},\"c\":-1}").expect("failed to parse json");
        assert_eq!(sum_json(&v), 3);
    }

    #[test]
    fn can_sum_nested_objects() {
        let v = serde_json::from_str(r#"[-1,{"a":1}]"#).expect("failed to parse json");
        assert_eq!(sum_json(&v), 0);
    }
}
