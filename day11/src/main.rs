fn main() {
    let mut monkeys = test_monkeys();
    make_rounds(&mut monkeys, true);

    let mut monkeys = real_monkeys();
    make_rounds(&mut monkeys, true);

    let mut monkeys = test_monkeys();
    make_rounds(&mut monkeys, false);

    let mut monkeys = real_monkeys();
    make_rounds(&mut monkeys, false);
}

fn make_rounds(mut monkeys: &mut Vec<Monkey>, part1: bool) {
    let mut divisor = 1;

    if part1 {
        divisor = 3
    } else {
        for m in 0..monkeys.len() {
            divisor *= monkeys[m].modulo;
        }
    }

    let nb = if part1 { 20 } else { 10_000 };
    for r in 1..nb + 1 {
        round(&mut monkeys, part1, divisor);
        println!();
        if part1 {
            println!("After round {r}, the monkeys are holding items with these worry levels:")
        } else {
            println!("== After round {r} ==")
        }

        for m in 0..monkeys.len() {
            if part1 {
                println!("Monkey {m}: {:?}", monkeys[m].items)
            } else {
                println!(
                    "Monkey {m} inspected items {:?} times",
                    monkeys[m].inspected
                )
            }
        }
        if part1 {
            println!()
        };
    }
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    for m in 0..monkeys.len() {
        println!("Monkey {m}: {:?}", monkeys[m].inspected);
    }

    let nb = if part1 { 1 } else { 2 };
    println!(
        "result {nb} : {} ",
        monkeys[0].inspected * monkeys[1].inspected
    )
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    modulo: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

fn round(monkeys: &mut Vec<Monkey>, part1: bool, divisor: usize) {
    for m in 0..monkeys.len() {
        if part1 {
            println!("Monkey {m}:")
        };
        monkeys[m].items.reverse();
        while let Some(item) = monkeys[m].items.pop() {
            monkeys[m].inspected += 1;
            if part1 {
                println!("\tMonkey inspects an item with a worry level of {}.", item)
            };
            let mut worry: usize = (monkeys[m].operation)(item);
            if part1 {
                println!("\t\tWorry level is now {}.", worry)
            };
            if part1 {
                worry = worry / divisor;
                println!(
                    "\t\tMonkey gets bored with item. Worry level is divided by 3 to {}.",
                    worry
                );
            } else {
                worry = worry % divisor;
            }
            if worry % monkeys[m].modulo == 0 {
                let if_true = monkeys[m].if_true;
                if part1 {
                    println!(
                        "\t\tCurrent worry level IS divisible by {}.",
                        monkeys[m].modulo
                    )
                };
                if part1 {
                    println!(
                        "\t\tItem with worry level {} is thrown to monkey {}.",
                        worry, if_true
                    )
                };
                monkeys[if_true].items.push(worry);
            } else {
                let if_false = monkeys[m].if_false;
                if part1 {
                    println!(
                        "\t\tCurrent worry level is not divisible by {}.",
                        monkeys[m].modulo
                    )
                };
                if part1 {
                    println!(
                        "\t\tItem with worry level {} is thrown to monkey {}.",
                        worry, if_false
                    )
                };
                monkeys[if_false].items.push(worry);
            }
        }
    }
}

fn test_monkeys() -> Vec<Monkey> {
    let mut l = vec![];

    l.push(Monkey {
        items: vec![79, 98],
        operation: Box::new(|a| a * 19),
        modulo: 23,
        if_true: 2,
        if_false: 3,
        inspected: 0,
    });

    l.push(Monkey {
        items: vec![54, 65, 75, 74],
        operation: Box::new(|a| a + 6),
        modulo: 19,
        if_true: 2,
        if_false: 0,
        inspected: 0,
    });

    l.push(Monkey {
        items: vec![79, 60, 97],
        operation: Box::new(|a| a * a),
        modulo: 13,
        if_true: 1,
        if_false: 3,
        inspected: 0,
    });

    l.push(Monkey {
        items: vec![74],
        operation: Box::new(|a| a + 3),
        modulo: 17,
        if_true: 0,
        if_false: 1,
        inspected: 0,
    });

    l
}

fn real_monkeys() -> Vec<Monkey> {
    let mut l = vec![];

    l.push(Monkey {
        items: vec![54, 89, 94],
        operation: Box::new(|a| a * 7),
        modulo: 17,
        if_true: 5,
        if_false: 3,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![66, 71],
        operation: Box::new(|a| a + 4),
        modulo: 3,
        if_true: 0,
        if_false: 3,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![76, 55, 80, 55, 55, 96, 78],
        operation: Box::new(|a| a + 2),
        modulo: 5,
        if_true: 7,
        if_false: 4,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![93, 69, 76, 66, 89, 54, 59, 94],
        operation: Box::new(|a| a + 7),
        modulo: 7,
        if_true: 5,
        if_false: 2,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![80, 54, 58, 75, 99],
        operation: Box::new(|a| a * 17),
        modulo: 11,
        if_true: 1,
        if_false: 6,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![69, 70, 85, 83],
        operation: Box::new(|a| a + 8),
        modulo: 19,
        if_true: 2,
        if_false: 7,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![89],
        operation: Box::new(|a| a + 6),
        modulo: 2,
        if_true: 0,
        if_false: 1,
        inspected: 0,
    });
    l.push(Monkey {
        items: vec![62, 80, 58, 57, 93, 56],
        operation: Box::new(|a| a * a),
        modulo: 13,
        if_true: 6,
        if_false: 4,
        inspected: 0,
    });

    l
}
