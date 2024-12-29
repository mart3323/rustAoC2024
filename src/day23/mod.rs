use std::cmp::Ordering::Greater;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use itertools::Itertools;
use crate::utils::read_input_file;

const DAY: &str = "day22";

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
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
        let halves: Vec<&str> = value.trim_end().split(",").collect();
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
            let mut l = l.split(",");
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
            let mut l = l.split(",");
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


#[test]
fn test_part2() {
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
            let mut l = l.split(",");
            let pc_a: NetAddr = l.next().unwrap().into();
            let pc_b: NetAddr = l.next().unwrap().into();
            mark_connected(pc_a, pc_b);
        });

    // Remove computers not connected to any historian
    let mut keep: HashSet<NetAddr> = connected.keys().map(|a| a.clone()).collect::<HashSet<NetAddr>>();
    for (addr, neighbors) in &connected {
        if !addr.0.starts_with("t") {
            if !neighbors.iter().any(|neighbor| neighbor.0.starts_with("t")) {
                keep.remove(addr);
            }
        }
    }
    let mut new_connected: HashMap<NetAddr, HashSet<NetAddr>> = HashMap::new();
    for (addr, neighbors) in connected {
        if keep.contains(&addr) {
            new_connected.insert(addr.clone(), neighbors.into_iter().filter(|n| keep.contains(n)).collect());
        }
    }
    let connected = new_connected;
    let max_possible_size = connected.iter()
        .filter(|&(pc, neighbors)| pc.0.starts_with("t"))
        .map(|(pc,neighbors)| neighbors.len()+1)
        .max()
        .expect("At least one historian still in the network");

    let password = (0..max_possible_size).rev()
        .flat_map(|possible_size|
            connected.iter().combinations(possible_size)
                .filter(|addresses| {
                    addresses.iter().all(|(addr1, connected1)|
                        addresses.iter().all(|(addr2, connected2)|
                            addr1 == addr2 || connected1.contains(addr2)
                        )
                    )
                })
        )
        .map(|pax| pax.into_iter().map(|(addr,_)| addr)
            .sorted()
            .map(|addr| addr.0.clone())
            .join(",")
        )
        .next();

    assert_eq!(password, Some(String::from("co,de,ka,ta")));;
}
pub fn part2() -> String {
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
            let mut l = l.split(",");
            let pc_a: NetAddr = l.next().unwrap().into();
            let pc_b: NetAddr = l.next().unwrap().into();
            mark_connected(pc_a, pc_b);
        });

    // Remove nodes not connected to any historian
    loop {
        let mut trimmed_something = false;

        let keys = connected.keys().map(|k| k.clone()).collect::<Vec<NetAddr>>();
        for k in keys {
            if !k.0.starts_with("t") {
                if !connected.get(&k).unwrap().iter().any(|b| b.0.starts_with("t")) {
                    trimmed_something = true;
                    connected.remove(&k);
                    connected.iter_mut().for_each(|(a,c)| {
                        c.remove(&k);
                    })
                }
            }
        }
        
        let keys = connected.keys().map(|k| k.clone()).collect::<Vec<NetAddr>>();
        for k in keys {
            if connected.get(&k).unwrap().len() < 5 {
                trimmed_something = true;
                connected.remove(&k);
                connected.iter_mut().for_each(|(a,c)| {
                    c.remove(&k);
                })
            }
        }
        if !trimmed_something {break}
    }
    
    
    let mut cliques_of_size: HashMap<usize, HashSet<String>> = HashMap::new();
    // Create cliques of size 1
    {
        let keys = connected.keys().map(|k| k.clone()).collect::<Vec<NetAddr>>();
        let mut cliques = HashSet::new();
        for key in keys {
            if key.0.starts_with("t") {
                cliques.insert(key.0.clone());
            }
        }
        cliques_of_size.insert(1, cliques);
    }
    // Expand existing cliques
    for size in 2usize.. {
        let mut cliques = HashSet::new();
        let keys = connected.keys().map(|k| k.clone()).collect::<Vec<NetAddr>>();
        let prev_cliques = cliques_of_size.get(&(size - 1)).expect("Previous iteration did not save");
        for prev in prev_cliques {
            let mut addresses = prev.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
            for key in keys.iter() {
                let c = connected.get(&key).unwrap();
                if addresses.iter().all(|addr| c.contains(&NetAddr(String::from(addr.clone())))) {
                    let mut new_addresses = addresses.clone();
                    new_addresses.push(key.0.clone());
                    new_addresses.sort();
                    cliques.insert(new_addresses.join(","));
                }
            }
        }
        if cliques.len() == 0 {
            break;
        }
        cliques_of_size.insert(size, cliques);
    }
    let largest_cliques = cliques_of_size.get(&cliques_of_size.len()).unwrap();
    assert_eq!(largest_cliques.len(), 1);
    return largest_cliques.iter().next().unwrap().clone();
    todo!();
    let max_possible_size = connected.iter()
        .filter(|&(pc, neighbors)| pc.0.starts_with("t"))
        .map(|(pc,neighbors)| neighbors.len()+1)
        .max()
        .expect("At least one historian still in the network");

    println!("max {}", max_possible_size);
    let password = (0..max_possible_size).rev()
        .filter_map(|possible_size|
            connected.iter()
                .filter(|(addr,_)| addr.0.starts_with("t")) // seed
                .flat_map(|(addr, neighbors)|
                    neighbors.iter().combinations(possible_size - 1)
                        .filter(|others| 
                            others.iter().all(|o1| 
                                 others.iter().all(|o2| 
                                     connected.get(o1).unwrap().contains(o2)
                                 )
                            )
                        )
                        .map(|mut combination| {
                            combination.push(addr);
                            return combination;
                        })
                )
                .next()
        )
        .map(|pax| pax.iter()
            .sorted()
            .map(|addr| addr.0.clone())
            .join(",")
        )
        .next();

    return password.expect("Found at least one fully connected clique with a historian in it");
}