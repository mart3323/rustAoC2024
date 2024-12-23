use std::cmp::Ordering::Greater;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::utils::read_input_file;

const DAY: &str = "day22";

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Hash)]
struct NetAddr(String);
impl From<&str> for NetAddr {
    fn from(addr: &str) -> Self {
        NetAddr(String::from(addr))
    }
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Connection {
    from: NetAddr,
    to: NetAddr
}
impl Connection {
    fn new(from: NetAddr, to: NetAddr) -> Self {
        match from.0.cmp(&to.0) {
            Greater => Connection { from, to },
            _ => Connection { from: to, to: from }
        }
    }
}
impl TryFrom<&str> for Connection {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let halves: Vec<&str> = value.trim_end().split("-").collect();
        if let Some(&from) = halves.get(0) {
            if let Some(&to) = halves.get(1) {
                if let None = halves.get(2) {
                    let from = NetAddr::from(from);
                    let to = NetAddr::from(to);
                    return Ok(Connection::new(from, to));
                }
            }
        }
        return Err(());
    }
}

#[test]
fn test_part1() {
    let string = read_input_file(DAY, "demo.txt");
    let mut connected: HashMap<NetAddr, HashSet<NetAddr>> = HashMap::new();
    let mut mark_connected = |a: NetAddr, b: NetAddr| {
        for (a,b) in [(&a,&b), (&b,&a)] {
            if let Some(set) = connected.get_mut(&a) {
                set.insert(b.clone());
            } else {
                let mut new_set = HashSet::new();
                new_set.insert(b.clone());
                connected.insert(a.clone(), new_set);
            }
        }
    };


    string.lines()
        .for_each(|l| {
            let l = l.trim_end();
            let mut l = l.split("-");
            let pc_a: NetAddr = l.next().unwrap().into();
            let pc_b: NetAddr = l.next().unwrap().into();
            mark_connected(pc_a, pc_b);
        });

    let total: usize = connected.keys()
        .flat_map(|a|
            connected.get(a).unwrap()
                .iter()
                .filter(move |b| a < b)
                .map(move |b| (a, b))
        )
        .flat_map(|(a,b)|
            connected.get(b).unwrap()
                .iter()
                .filter(move |c| b < c)
                .map(move |c| (a, b, c))
        )
        .filter(|(a,b,c)| a < b && b < c)
        .filter(|(a,b,c)| connected.get(a).unwrap().contains(c))
        .filter(|(a,b,c)| {
            return a.0.starts_with("t") || b.0.starts_with("t") || c.0.starts_with("t")
        })
        .count();

    assert_eq!(total, 7);
}

pub fn part1() -> usize {
    let string = read_input_file(DAY, "full.txt");
    let mut connected: HashMap<NetAddr, HashSet<NetAddr>> = HashMap::new();
    let mut mark_connected = |a: NetAddr, b: NetAddr| {
        for (a,b) in [(&a,&b), (&b,&a)] {
            if let Some(set) = connected.get_mut(&a) {
                set.insert(b.clone());
            } else {
                let mut new_set = HashSet::new();
                new_set.insert(b.clone());
                connected.insert(a.clone(), new_set);
            }
        }
    };


    string.lines()
        .for_each(|l| {
            let l = l.trim_end();
            let mut l = l.split("-");
            let pc_a: NetAddr = l.next().unwrap().into();
            let pc_b: NetAddr = l.next().unwrap().into();
            mark_connected(pc_a, pc_b);
        });

    let total: usize = connected.keys()
        .flat_map(|a|
            connected.get(a).unwrap()
                .iter()
                .filter(move |b| a < b)
                .map(move |b| (a, b))
        )
        .flat_map(|(a,b)|
            connected.get(b).unwrap()
                .iter()
                .filter(move |c| b < c)
                .map(move |c| (a, b, c))
        )
        .filter(|(a,b,c)| a < b && b < c)
        .filter(|(a,b,c)| connected.get(a).unwrap().contains(c))
        .filter(|(a,b,c)| {
            return a.0.starts_with("t") || b.0.starts_with("t") || c.0.starts_with("t")
        })
        .count();

    return total;
}