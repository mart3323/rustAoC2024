use std::cmp::min;
use std::fmt::{Display, Formatter};
use cached::proc_macro::cached;

const DAY: &str = "day20";

fn cost_move(dx: i8, dy: i8) -> usize {
    let mut mv_cost = 0;
    if dx < 0 {
        mv_cost = 3
    } else if dy > 0 {
        mv_cost = 2
    } else if dx == 0 && dy < 0 || dx > 0 && dy == 0 {
        mv_cost = 1
    } else if dx == 0 && dy == 0 {
        mv_cost = 0;
    } else {
        panic!()
    }
    return mv_cost
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum KeypadKey {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D0,
    A
}
impl KeypadKey {
    #[rustfmt::skip]
    fn position(&self) -> (usize, usize) {
        use KeypadKey::*;
        match self {
            D7 => (0,0), D8 => (1,0), D9 => (2,0),
            D4 => (0,1), D5 => (1,1), D6 => (3,1),
            D1 => (0,2), D2 => (1,2), D3 => (2,2),
                         D0 => (1,3),  A => (2,3),
        }
    }
}
fn cost_to_input_code(input: Vec<KeypadKey>, depth: usize) -> usize {
    let mut pos = KeypadKey::A.position();
    let mut cost = 0;
    for digit in input {
        let target = digit.position();
        let dx = target.0 as i8 - pos.0 as i8;
        let dy = target.1 as i8 - pos.1 as i8;
        if dx == 0 && dy == 0 {
        } else if dx < 0 {
            cost += cost_to_press_key(DirpadKey::Left, depth) + dy.abs() as usize + dx.abs() as usize - 1;
        } else if dy > 0 {
            cost += cost_to_press_key(DirpadKey::Down, depth) + dy.abs() as usize + dx.abs() as usize - 1;
        } else if dx > 0 && dy < 0 {
            cost += dy.abs() as usize + dx.abs() as usize - 2;
            cost += cost_to_press_key(DirpadKey::Right, depth);
            cost += cost_to_press_key(DirpadKey::Up, depth);
        } else if dx > 0 {
            cost += dy.abs() as usize + dx.abs() as usize - 1;
            cost += cost_to_press_key(DirpadKey::Right, depth);
        } else if dy < 0 {
            cost += dy.abs() as usize + dx.abs() as usize - 1;
            cost += cost_to_press_key(DirpadKey::Up, depth);
        } else {
            panic!();
        }
        cost += 1;
        println!("{:?}, {} ({}, {})", digit, cost, dx, dy);
        pos = target;
    };
    return cost;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum DirpadKey {
    Up=0,
    Down=1,
    Left=2,
    Right=3,
    A =4,
}
impl Display for DirpadKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DirpadKey::Up => "^",
            DirpadKey::Down => "v",
            DirpadKey::Left => "<",
            DirpadKey::Right => ">",
            DirpadKey::A => "A",
        })
    }
}
#[cached]
/// Given a sequence of 'depth' keypads, return the number of presses required on the first keypad
/// in order to press the given key on the last keypad.
///
/// A depth of 0 corresponds to having direct access to the output keypad
/// A depth of 1 corresponds to having one robot controlled keypad, followed by the one you have direct control over
fn cost_to_press_key(btn: DirpadKey, depth: usize) -> usize {
    use DirpadKey::*;
    // println!("ctpk({}, {})", btn, depth);
    if depth == 0 {
        1
    } else if btn == A {
        1
    } else {
        let next = depth - 1;
        let nextCost = |dir| cost_to_press_key(dir, next);
        match btn {
            A => nextCost(A),
            Up => [Left, A, Right, A].map(nextCost).iter().sum::<usize>() + 1,
            Right => [Down, A, Up, A].map(nextCost).iter().sum::<usize>() + 1,
            Down => {
                let option1 = [Left, Down, A, Right, Up, A].map(nextCost).iter().sum::<usize>();
                let option2 = [Down, Left, A, Right, Up, A].map(nextCost).iter().sum::<usize>();
                let option3 = [Down, Left, A, Up, Right, A].map(nextCost).iter().sum::<usize>();
                let option4 = [Left, Down, A, Up, Right, A].map(nextCost).iter().sum::<usize>();
                return min(min(min(option1, option2), option3), option4) + 1;
            },
            Left => {
                let option1 = [Left, Down, Left, A, Right, Up, Right, A].map(nextCost).iter().sum::<usize>();
                let option2 = [Down, Left, Left, A, Right, Up, Right, A].map(nextCost).iter().sum::<usize>();
                let option3 = [Down, Left, Left, A, Right, Right, Up, A].map(nextCost).iter().sum::<usize>();
                let option4 = [Left, Down, Left, A, Right, Up, Right, A].map(nextCost).iter().sum::<usize>();
                return min(min(min(option1, option2), option3), option4) + 1;
            },
        }
    }
}

fn parse_code(s: &str) -> Vec<KeypadKey> {
    s.chars().map(|c| match c {
        '0' => KeypadKey::D0,
        '1' => KeypadKey::D1,
        '2' => KeypadKey::D2,
        '3' => KeypadKey::D3,
        '4' => KeypadKey::D4,
        '5' => KeypadKey::D5,
        '6' => KeypadKey::D6,
        '7' => KeypadKey::D7,
        '8' => KeypadKey::D8,
        '9' => KeypadKey::D9,
        'A' => KeypadKey::A,
        _ => panic!(),
    }).collect()
}

#[test]
fn test_costs() {

    let codes = vec!("029A", "980A", "179A", "456A", "379A");
    for code in codes {
        let c = parse_code(code);
        println!("{}: {}", code, cost_to_input_code(c, 1));
    }


    use DirpadKey::*;
    for depth in 1..5 {
        println!("---");
        for btn in [A, Right, Up, Left, Down] {
            let cost = cost_to_press_key(btn, depth);
            println!("At depth {}, button {} costs {}", depth, btn, cost)
        }
    }
}
