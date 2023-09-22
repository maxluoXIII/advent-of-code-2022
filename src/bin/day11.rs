use evalexpr::*;
use sscanf::sscanf;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Read},
};

struct Item {
    worry: i64,
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        Item {
            worry: value
                .parse::<i64>()
                .expect("Unable to parse worry value for item"),
        }
    }
}

struct Monkey {
    id: usize,
    items: VecDeque<Item>,
    op_expr: evalexpr::Node,
    test_num: i64,
    true_target: usize,
    false_target: usize,
    inspect_count: u64,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut line_iter = value.lines();
        let id = sscanf!(line_iter.next().unwrap(), "Monkey {usize}:")
            .expect("Unable to parse monkey id");
        let item_str = sscanf!(line_iter.next().unwrap(), "  Starting items: {str}")
            .expect("Unable to parse items");
        let items: VecDeque<Item> = item_str.split(", ").map(Item::from).collect();
        let op_str = sscanf!(line_iter.next().unwrap(), "  Operation: new = {str}")
            .expect("Unable to parse op string");
        let op_expr = evalexpr::build_operator_tree(op_str)
            .expect("Could not parse operation from op string");
        let test_num = sscanf!(line_iter.next().unwrap(), "  Test: divisible by {i64}")
            .expect("Unable to parse test num");
        let true_target = sscanf!(
            line_iter.next().unwrap(),
            "    If true: throw to monkey {usize}"
        )
        .expect("Unable to parse true target");
        let false_target = sscanf!(
            line_iter.next().unwrap(),
            "    If false: throw to monkey {usize}"
        )
        .expect("Unable to parse false target");

        Monkey {
            id,
            items,
            op_expr,
            test_num,
            true_target,
            false_target,
            inspect_count: 0,
        }
    }
}

fn calc_monkey_business(monkeys: &HashMap<usize, Monkey>) -> u64 {
    let most_active_monkey = monkeys
        .values()
        .max_by_key(|monkey| monkey.inspect_count)
        .unwrap();
    let second_most_active_monkey = monkeys
        .values()
        .max_by_key(|monkey| {
            if monkey.id == most_active_monkey.id {
                u64::MIN
            } else {
                monkey.inspect_count
            }
        })
        .unwrap();

    most_active_monkey.inspect_count * second_most_active_monkey.inspect_count
}

fn main() {
    let file = File::open("data/day11-full.txt").expect("Could not find date file");
    let mut reader = BufReader::new(file);

    let mut all_monkey_str = String::new();
    reader
        .read_to_string(&mut all_monkey_str)
        .expect("Could not read file");

    let mut monkeys: HashMap<usize, Monkey> = all_monkey_str
        .split("\n\n")
        .map(Monkey::from)
        .enumerate()
        .collect();
    let monkey_count = monkeys.len();

    let common_test_divisor: i64 = monkeys.values().map(|monkey| monkey.test_num).product();

    let mut round_count = 0;
    while round_count < 10_000 {
        round_count += 1;
        for i in 0..monkey_count {
            let mut monkey = monkeys.remove(&i).unwrap();
            while !monkey.items.is_empty() {
                monkey.inspect_count += 1;
                let mut item = monkey.items.pop_front().unwrap();
                let mut context = HashMapContext::new();
                context.set_value("old".into(), item.worry.into()).unwrap();

                let new_worry = monkey
                    .op_expr
                    .eval_with_context(&context)
                    .unwrap()
                    .as_int()
                    .unwrap();
                item.worry = new_worry % common_test_divisor;
                // item.worry /= 3;

                let mut monkey_target = &monkey.false_target;
                if item.worry % monkey.test_num == 0 {
                    monkey_target = &monkey.true_target;
                }
                monkeys
                    .get_mut(monkey_target)
                    .unwrap()
                    .items
                    .push_back(item);
            }
            monkeys.insert(monkey.id, monkey);
        }
    }

    println!("Monkey business: {}", calc_monkey_business(&monkeys));
}
