use crate::List::{Cons, Nil};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Eq)]
enum List {
    Cons(ListItem, Box<List>),
    Nil,
}

fn find_matching_bracket(haystack: &str, open_pos: usize) -> usize {
    let mut open_bracket_count = 0;
    let mut close_bracket_count = 0;
    let mut ret = 0;
    for (idx, c) in haystack[open_pos..].chars().enumerate() {
        if c == '[' {
            open_bracket_count += 1;
        } else if c == ']' {
            close_bracket_count += 1;
            if open_bracket_count == close_bracket_count {
                ret = idx;
                break;
            }
        }
    }

    ret
}

impl From<&str> for List {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            return Nil;
        }

        let inner_str =
            if value.starts_with('[') && find_matching_bracket(value, 0) == value.len() - 1 {
                &value[1..value.len() - 1]
            } else {
                value
            };
        if inner_str.is_empty() {
            return Nil;
        }

        let mut end_idx = inner_str.find(',').unwrap_or(inner_str.len());
        if inner_str.starts_with('[') {
            end_idx = find_matching_bracket(inner_str, 0) + 1;
        }
        let (element_str, rest_str) = inner_str.split_at(end_idx);
        let last_inner_regex = Regex::new(r"^\d+]+$").unwrap();
        let clear_idx = if last_inner_regex.is_match(element_str) {
            element_str.find(']').unwrap()
        } else {
            element_str.len()
        };
        let this_item = ListItem::from(&element_str[..clear_idx]);

        let (next_start_idx, _) = rest_str
            .chars()
            .find_position(|c| c.is_ascii_digit() || *c == '[')
            .unwrap_or((rest_str.len(), '_'));
        Cons(this_item, Box::new(List::from(&rest_str[next_start_idx..])))
    }
}

impl TryFrom<&ListItem> for List {
    type Error = &'static str;

    fn try_from(value: &ListItem) -> Result<Self, Self::Error> {
        match value {
            ListItem::Item(num) => Ok(Cons(ListItem::Item(*num), Box::new(Nil))),
            ListItem::List(_) => Err("Cannot move/copy List to List"),
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cons(left_item, left_list), Cons(right_item, right_list)) => {
                left_item == right_item && left_list == right_list
            }
            (Nil, Cons(_, _)) => false,
            (Cons(_, _), Nil) => false,
            (Nil, Nil) => true,
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Cons(left_item, left_list), Cons(right_item, right_list)) => {
                match left_item.cmp(right_item) {
                    Ordering::Equal => left_list.cmp(right_list),
                    unequal => unequal,
                }
            }
            (Nil, Cons(_, _)) => Ordering::Less,
            (Cons(_, _), Nil) => Ordering::Greater,
            (Nil, Nil) => Ordering::Equal,
        })
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq)]
enum ListItem {
    List(Box<List>),
    Item(u32),
}

impl From<&str> for ListItem {
    fn from(value: &str) -> Self {
        if value.starts_with('[') {
            ListItem::List(Box::new(List::from(value)))
        } else {
            ListItem::Item(value.parse::<u32>().unwrap())
        }
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListItem::List(left_list), ListItem::List(right_list)) => left_list == right_list,
            (ListItem::List(left_list), right_item @ ListItem::Item(_)) => {
                **left_list == List::try_from(right_item).unwrap()
            }
            (left_item @ ListItem::Item(_), ListItem::List(right_list)) => {
                **right_list == List::try_from(left_item).unwrap()
            }
            (left_item @ ListItem::Item(_), right_item @ ListItem::Item(_)) => {
                left_item == right_item
            }
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ListItem::List(left_list), ListItem::List(right_list)) => {
                left_list.partial_cmp(right_list)
            }
            (ListItem::List(left_list), right_item @ ListItem::Item(_)) => {
                (**left_list).partial_cmp(&List::try_from(right_item).unwrap())
            }
            (left_item @ ListItem::Item(_), ListItem::List(right_list)) => {
                List::try_from(left_item).unwrap().partial_cmp(right_list)
            }
            (ListItem::Item(left_num), ListItem::Item(right_num)) => {
                left_num.partial_cmp(right_num)
            }
        }
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct ListPair {
    list1: List,
    list2: List,
}

impl From<&str> for ListPair {
    fn from(value: &str) -> Self {
        let mut line_iter = value.lines();
        ListPair {
            list1: List::from(line_iter.next().unwrap()),
            list2: List::from(line_iter.next().unwrap()),
        }
    }
}

impl ListPair {
    fn is_ordered(&self) -> bool {
        self.list1 <= self.list2
    }
}

fn main() {
    let file = File::open("data/day13-full.txt").expect("Couldn't find data file");
    let mut reader = BufReader::new(file);

    let mut all_list_str = String::new();
    reader
        .read_to_string(&mut all_list_str)
        .expect("Could not read file");
    let ordered_index_sum = all_list_str
        .split("\n\n")
        .map(ListPair::from)
        .map(|pair| pair.is_ordered())
        .enumerate()
        .filter(|(_, ordered)| *ordered)
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    println!("Ordered index sum: {ordered_index_sum}");
}
