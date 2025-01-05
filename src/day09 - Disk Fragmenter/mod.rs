use nom::character::char;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use utils::read_input_file;

const DAY: &'static str = "day09 - Disk Fragmenter";

type Frequency = char;
type Antenna = Frequency;
type Strength = usize;

fn char_to_usize(ch: &char) -> usize {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!(),
    }
}
#[derive(Debug, Clone)]
struct FreeRegion {
    len: usize,
}
#[derive(Debug, Clone)]
struct UsedRegion {
    pid: usize,
    len: usize,
}
#[derive(Clone)]
enum DiskRegion {
    Free(FreeRegion),
    Used(UsedRegion),
}
impl Debug for DiskRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskRegion::Free(FreeRegion { len }) => {
                write!(f, "({:?})", len)
            }
            DiskRegion::Used(UsedRegion { pid, len }) => {
                write!(f, "({:?}, {:?})", pid, len)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct DiskMap {
    regions: Vec<DiskRegion>,
}
impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for region in &self.regions {
            match region {
                DiskRegion::Free(FreeRegion { len }) => {
                    for _ in 0..*len {
                        s.push('.');
                    }
                },
                DiskRegion::Used(UsedRegion { pid, len }) => {
                    for _ in 0..*len {
                        s.push_str(&format!("{}", pid));
                    }
                },
            }
        }
        return write!(f, "{}", s);
    }
}
impl DiskMap {
    fn from_compressed_string(str: &str) -> DiskMap {
        DiskMap {
            regions: str
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let len = char_to_usize(&c);
                    if i % 2 == 0 {
                        let pid = i / 2;
                        return DiskRegion::Used(UsedRegion { pid, len });
                    } else {
                        return DiskRegion::Free(FreeRegion { len });
                    }
                })
                .collect(),
        }
    }
    fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut pos = 0;
        for region in self.regions.iter() {
            match region {
                DiskRegion::Free(free) => {
                    pos += free.len;
                }
                DiskRegion::Used(UsedRegion { pid, len }) => {
                    for i in pos..(pos + len) {
                        checksum += i * pid;
                    }
                    pos = pos + len;
                }
            }
        }
        return checksum;
    }
}

fn compress_simple(input: &DiskMap) -> DiskMap {
    let mut input = VecDeque::from(input.regions.clone());
    let mut output: Vec<UsedRegion> = Vec::new();
    while let Some(ref front) = input.pop_front() {
        match front {
            DiskRegion::Used(ref used) => {
                output.push(used.clone());
            }
            DiskRegion::Free(ref free) => {
                while let Some(ref back) = input.pop_back() {
                    if let DiskRegion::Used(used) = back {
                        if used.len > free.len {
                            output.push(UsedRegion {
                                pid: used.pid,
                                len: free.len,
                            });
                            input.push_back(DiskRegion::Used(UsedRegion {
                                pid: used.pid,
                                len: used.len - free.len,
                            }));
                            break;
                        } else {
                            output.push(used.clone());
                            input.push_front(DiskRegion::Free(FreeRegion {
                                len: free.len - used.len,
                            }));
                            break;
                        }
                    }
                }
            }
        }
    }
    return DiskMap {
        regions: output
            .iter()
            .map(|r| {
                DiskRegion::Used(UsedRegion {
                    pid: r.pid,
                    len: r.len,
                })
            })
            .collect(),
    };
}
fn fmt_diskregions(d: &Vec<DiskRegion>) -> String {
    d.iter().map(|r| match r {
        DiskRegion::Free(FreeRegion{len}) => format!("{}", "_".repeat(*len)),
        DiskRegion::Used(UsedRegion{len, pid}) => format!("{}", (pid.to_string().repeat(*len)))
    }).collect()
}
fn fmt_vecdeque(d: &VecDeque<DiskRegion>) -> String {
    d.iter().map(|r| match r {
        DiskRegion::Free(FreeRegion{len}) => format!("{}", "_".repeat(*len)),
        DiskRegion::Used(UsedRegion{len, pid}) => format!("{}", (pid.to_string().repeat(*len)))
    }).collect()
}
fn compress_advanced(input: &DiskMap) -> DiskMap {
    let mut output: Vec<DiskRegion> = input.regions.clone();
    
    let mut processed_pids_above = usize::MAX;
    let mut i = output.len();
    'cursor_right:
    while 0 <= i {
        if let Some(DiskRegion::Used(UsedRegion { pid, len })) = output.get(i) {
            if *pid < processed_pids_above {
                processed_pids_above = *pid;
                
                let len = *len;
                let mut j = 0;
                'cursor_left:
                while j < i {
                    if let Some(DiskRegion::Free(FreeRegion { len: free_space })) = output.get_mut(j) {
                        if len <= *free_space {
                            // Reduce free space
                            *free_space -= len;
                            let free_space = *free_space;
                            // Move DiskRegion
                            let item = output.remove(i);
                            output.insert(i, DiskRegion::Free(FreeRegion { len }));
                            output.insert(j, item);
                            if free_space == 0 {
                                output.remove(j + 1);
                                i -= 1;
                            }
                            i -= 1;
                            // Do not move the right cursor, we just rotated a new sector to the same index
                            continue 'cursor_right;
                        }
                    }
                    j += 1;
                }
            }
        }
        if i == 0 {
            break
        } else {
            i -= 1;
        }
    }
    return DiskMap { regions: output};
}

#[test]
fn test_compress_simple() {
    let demo_txt = read_input_file(DAY, "demo.txt");
    let demo_input = DiskMap::from_compressed_string(&demo_txt);
    let compacted = compress_simple(&demo_input);
    assert_eq!(compacted.checksum(), 1928usize);
}

#[test]
fn test_compress_advanced() {
    let demo_txt = read_input_file(DAY, "demo.txt");
    let demo_input = DiskMap::from_compressed_string(&demo_txt);
    let compacted = compress_advanced(&demo_input);
    println!("{:?}", compacted);
    assert_eq!(compacted.checksum(), 2858usize);
}

pub fn part1() -> usize {
    let full_txt = read_input_file(DAY, "full.txt");
    let full_input = DiskMap::from_compressed_string(&full_txt);
    let compacted = compress_simple(&full_input);
    compacted.checksum()
}
pub fn part2() -> usize {
    let full_txt = read_input_file(DAY, "full.txt");
    let full_input = DiskMap::from_compressed_string(&full_txt);
    let compacted = compress_advanced(&full_input);
    compacted.checksum()
}
