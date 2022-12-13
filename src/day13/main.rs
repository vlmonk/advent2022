use std::cmp::Ordering;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Item {
    pub fn empty_list() -> Self {
        Self::List(vec![])
    }

    pub fn num(num: i32) -> Self {
        Self::Number(num)
    }

    pub fn parse(input: &str) -> Self {
        let result = Item::take_list(input);
        match result {
            Some((item, _)) => item,
            _ => panic!("Can't parse"),
        }
    }

    fn take_list(input: &str) -> Option<(Item, &str)> {
        let first_char = input.chars().next();

        match first_char {
            Some('[') => {
                let (list, r) = Item::take_comma_list(&input[1..]);
                if let Some(']') = r.chars().next() {
                    let item = Item::List(list);
                    let rest = &r[1..];
                    Some((item, rest))
                } else {
                    panic!("Can't find ]")
                }
            }
            _ => None,
        }
    }

    fn take_number(input: &str) -> Option<(Item, &str)> {
        let num = input
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>();

        if num.len() > 0 {
            let number = num.parse::<i32>().expect("wrong number");
            Some((Item::Number(number), &input[num.len()..]))
        } else {
            None
        }
    }

    fn take_item(input: &str) -> Option<(Item, &str)> {
        Item::take_list(input).or_else(|| Item::take_number(input))
    }

    fn take_comma_list(input: &str) -> (Vec<Item>, &str) {
        let mut result = vec![];
        let mut rest = input;

        loop {
            match Item::take_item(rest) {
                Some((i, r)) => {
                    result.push(i);

                    if Some(',') == r.chars().next() {
                        rest = &r[1..];
                    } else {
                        rest = r;
                        break;
                    }
                }
                None => break,
            }
        }

        return (result, rest);
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::List(a), Item::List(b)) => cmp_list(&a, &b),
            (Item::Number(a), Item::List(b)) => cmp_list(&[Item::num(*a)], &b),
            (Item::List(a), Item::Number(b)) => cmp_list(&a, &[Item::num(*b)]),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_list(a: &[Item], b: &[Item]) -> Ordering {
    for (idx, ai) in a.iter().enumerate() {
        let bi = b.get(idx);

        match bi {
            Some(bi) => match ai.partial_cmp(bi) {
                Some(Ordering::Less) => return Ordering::Less,
                Some(Ordering::Greater) => return Ordering::Greater,
                _ => {}
            },
            None => return Ordering::Greater,
        }
    }

    if a.len() == b.len() {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

#[derive(Debug)]
struct Pair {
    a: Item,
    b: Item,
}

impl Pair {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let a = lines.next().expect("Missing first line");
        let b = lines.next().expect("Missing second line");

        Self {
            a: Item::parse(a),
            b: Item::parse(b),
        }
    }
}

fn main() -> Result<()> {
    let raw = advent2022::read_input()?;
    let pairs = raw
        .split("\n\n")
        .map(|i| Pair::parse(i))
        .collect::<Vec<_>>();

    let task_a: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, p)| if p.a < p.b { Some(idx + 1) } else { None })
        .sum();

    println!("Task a: {task_a}");

    let mut total = pairs
        .into_iter()
        .map(|p| [p.a, p.b])
        .flatten()
        .collect::<Vec<_>>();

    total.push(Item::parse("[[2]]"));
    total.push(Item::parse("[[6]]"));
    total.sort();

    let sample_a = Item::parse("[[2]]");
    let key_a = total
        .iter()
        .enumerate()
        .find_map(|(idx, i)| if i == &sample_a { Some(idx + 1) } else { None })
        .expect("NOT FOUND A");

    let sample_b = Item::parse("[[6]]");
    let key_b = total
        .iter()
        .enumerate()
        .find_map(|(idx, i)| if i == &sample_b { Some(idx + 1) } else { None })
        .expect("NOT FOUND A");

    let task_b = key_a * key_b;
    println!("Task b: {task_b}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let input = "[]";
        let result = Item::parse(&input);
        let expected = Item::List(vec![]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty_inner() {
        let input = "[[]]";
        let result = Item::parse(input);
        let expected = Item::List(vec![Item::List(vec![])]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty_inner2() {
        let input = "[[[]]]";
        let result = Item::parse(input);
        let expected = Item::List(vec![Item::List(vec![Item::List(vec![])])]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_comma() {
        let input = "[[],[]]";
        let result = Item::parse(input);
        let expected = Item::List(vec![Item::List(vec![]), Item::List(vec![])]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_comma2() {
        let input = "[[[]],[],[]]";
        let result = Item::parse(input);
        let expected = Item::List(vec![
            Item::List(vec![Item::empty_list()]),
            Item::empty_list(),
            Item::empty_list(),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_number() {
        let input = "[123,10,[11]]";
        let result = Item::parse(input);
        let expected = Item::List(vec![
            Item::Number(123),
            Item::Number(10),
            Item::List(vec![Item::Number(11)]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cmp() {
        let a = Item::Number(10);
        let b = Item::Number(5);

        assert!(a > b);
    }

    #[test]
    fn test_cmp_list() {
        let a = Item::parse("[5]");
        let b = Item::parse("[10]");

        assert!(a < b);
    }

    #[test]
    fn test_cmp_list2() {
        let a = Item::parse("[5,6]");
        let b = Item::parse("[5,7]");

        assert!(a < b);
    }

    #[test]
    fn test_cmp_list3() {
        let a = Item::parse("[5,6]");
        let b = Item::parse("[5]");

        assert!(a > b);
    }

    #[test]
    fn test_cmp_list4() {
        let a = Item::parse("[5,6]");
        let b = Item::parse("[5,6,1]");

        assert!(a < b);
    }

    #[test]
    fn test_cmp_list_with_num() {
        let a = Item::parse("[[1],2]");
        let b = Item::parse("[1,[3]]");

        assert!(a < b);
    }
}
