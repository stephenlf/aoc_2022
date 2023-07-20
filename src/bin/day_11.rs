use advent_of_code::day_11::*;

fn main() {
    // let monkeys = parse_lines("inputs/11.inputs.txt").unwrap();

    // for _ in 0..20 {
    //     for monkey in monkeys.iter() {
    //         let items = monkey.borrow_mut().throw_items();
    //         for (item, monkey_id) in items {
    //             monkeys[monkey_id].borrow_mut().items.push_back(item);
    //         }
    //     }
    // }
    
    // let mut sorted_checked_items = monkeys.iter()
    //     .map(|monkey| monkey.borrow().checked)
    //     .collect::<Vec<u128>>();
    // sorted_checked_items.sort();

    // let max = sorted_checked_items.pop().unwrap();
    // let next_max = sorted_checked_items.pop().unwrap();

    // part 2:
    let monkeys = big::parse_lines("inputs/11.inputs.txt");

    for i in 1..=10000 {
        for monkey in monkeys.iter() {
            let items = monkey.borrow_mut().throw_items();
            for (item, monkey_id) in items {
                monkeys[monkey_id].borrow_mut().items.push_back(item);
            }
        }
        if i % 1000 == 0 || i == 1 || i == 20 {
            for monkey in monkeys.iter() {
                monkey.borrow().list_items();
            }
        }
    }

}
