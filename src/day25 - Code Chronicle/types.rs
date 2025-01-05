
// Unique key/lock definitions:
// Each column can be between 0 and 5 high
// 5 columns total
// 6^5 = 7776 unique combinations
// u8: too small, max 255
// u16: Plenty of space, surely

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LockPattern(u16);
type LockPatternSplit = (u8,u8,u8,u8,u8);
impl From<&LockPatternSplit> for LockPattern {
    fn from(tuple: &LockPatternSplit) -> Self {
        let (a,b,c,d,e) = *tuple;
        LockPattern(
            a as u16 * 6u16.pow(4) +
                b as u16 * 6u16.pow(3) +
                c as u16 * 6u16.pow(2) +
                d as u16 * 6u16.pow(1) +
                e as u16 * 6u16.pow(0)
        )
    }
}
impl From<&LockPattern> for LockPatternSplit {
    fn from(pattern: &LockPattern) -> Self {
        (
            ((pattern.0 / 6u16.pow(4)) % 6) as u8,
            ((pattern.0 / 6u16.pow(3)) % 6) as u8,
            ((pattern.0 / 6u16.pow(2)) % 6) as u8,
            ((pattern.0 / 6u16.pow(1)) % 6) as u8,
            ((pattern.0 / 6u16.pow(0)) % 6) as u8,
        )
    }
}
impl LockPattern {
    pub fn iter_fitting_key_patterns(&self) -> impl Iterator<Item=LockPattern> {
        let (a,b,c,d,e): LockPatternSplit = self.into();
        (0..=(5-a)).flat_map(move |i|
            (0..=(5-b)).flat_map(move |j|
                (0..=(5-c)).flat_map(move |k|
                    (0..=(5-d)).flat_map(move |l|
                        (0..=(5-e)).map(move |m|
                            LockPattern::from(&(i,j,k,l,m))
                        ).into_iter()
                    ).into_iter()
                ).into_iter()
            ).into_iter()
        ).into_iter()
    }
}


#[test]
fn test_lock_pattern_encoding_survives_round_trip() {
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        assert_eq!((a,b,c,d,e), LockPatternSplit::from(&LockPattern::from(&(a,b,c,d,e))))
                    }
                }
            }
        }
    }
}

pub struct LockCompatibilityTable([usize; 6usize.pow(5)]);

impl LockCompatibilityTable {
    pub fn new() -> Self {
        LockCompatibilityTable([0; 6usize.pow(5)])
    }
    pub fn increment_fitting(&mut self, lock: LockPattern) {
        lock.iter_fitting_key_patterns().for_each(|p| {
            self.0[p.0 as usize] += 1
        })
    }
    pub fn get_fitting_count(&self, key: LockPattern) -> usize {
        self.0[key.0 as usize]
    }
}

#[test]
fn test_lock_compatibility_table() {
    let mut compat_table = LockCompatibilityTable::new();
    compat_table.increment_fitting(LockPattern::from(&(0,5,3,4,3)));
    compat_table.increment_fitting(LockPattern::from(&(1,2,0,5,3)));

    assert_eq!(compat_table.get_fitting_count(LockPattern::from(&(5,0,2,1,3))), 0);
    assert_eq!(compat_table.get_fitting_count(LockPattern::from(&(4,3,4,0,2))), 1);
    assert_eq!(compat_table.get_fitting_count(LockPattern::from(&(3,0,2,0,1))), 2);
}
