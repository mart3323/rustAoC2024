use std::fmt::Display;

trait Layout<T> {
    fn left(&self) -> Option<T>;
    fn right(&self) -> Option<T>;
    fn up(&self) -> Option<T>;
    fn down(&self) -> Option<T>;
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum NumpadKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
#[rustfmt::skip]
impl Layout<NumpadKey> for NumpadKey {
    fn left(&self) -> Option<NumpadKey> {
        use NumpadKey::*;
        match self  {
            Seven => None, Eight => Some(Seven), Nine => Some(Eight),
            Four  => None,  Five => Some(Four),   Six => Some(Five),
            One   => None,   Two => Some(One),  Three => Some(Two),
                            Zero => None,           A => Some(Zero),
        }
    }
    fn right(&self) -> Option<NumpadKey> {
        use NumpadKey::*;
        match self  {
            Seven => Some(Eight), Eight => Some(Nine),  Nine  => None,
            Four  => Some(Five),  Five  => Some(Six),   Six   => None,
            One   => Some(Two),   Two   => Some(Three), Three => None,
                                   Zero => Some(A),         A => None,
        }
    }
    fn up(&self) -> Option<NumpadKey> {
        use NumpadKey::*;
        match self  {
            Seven => None,       Eight => None,        Nine => None,
            Four  => Some(Seven), Five => Some(Eight),  Six => Some(Nine),
            One   => Some(Four),   Two => Some(Five), Three => Some(Six),
                                  Zero => Some(Two),      A => Some(Three),
        }
    }
    fn down(&self) -> Option<NumpadKey> {
        use NumpadKey::*;
        match self  {
            Seven => Some(Four), Eight => Some(Five),  Nine => Some(Six),
            Four  => Some(One),   Five => Some(Two),    Six => Some(Three),
            One   => None,         Two => Some(Zero), Three => Some(A),
                                  Zero => None,           A => None,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum DirpadKey {
    A,
    Up,
    Right,
    Down,
    Left,
}
#[rustfmt::skip]
impl Layout<DirpadKey> for DirpadKey {
    fn left(&self) -> Option<DirpadKey> {
        use DirpadKey::*;
        match self {
                            Up => None,           A => Some(Up),
            Left => None, Down => Some(Left), Right => Some(Down)
        }
    }

    fn right(&self) -> Option<DirpadKey> {
        use DirpadKey::*;
        match self {
                                  Up => Some(A),         A => None,
            Left => Some(Down), Down => Some(Right), Right => None
        }
    }

    fn up(&self) -> Option<DirpadKey> {
        use DirpadKey::*;
        match self {
                            Up => None,         A => None,
            Left => None, Down => Some(Up), Right => Some(A)
        }
    }

    fn down(&self) -> Option<DirpadKey> {
        use DirpadKey::*;
        match self {
                            Up => Some(Down),  A => Some(Right),
            Left => None, Down => None,    Right => None
        }
    }
}

pub trait Robot {
    fn input(&mut self, key: DirpadKey) -> Result<Option<NumpadKey>, ()>;
}
#[derive(Debug, Clone)]
pub struct NumpadRobot {
    pub key: NumpadKey,
}
#[derive(Debug, Clone)]
pub struct DirpadRobot<NextBot: Robot+Clone> {
    pub key: DirpadKey,
    pub controls: NextBot
}
impl<NextBot: Robot+Clone> Robot for DirpadRobot<NextBot> {
    fn input(&mut self, key: DirpadKey) -> Result<Option<NumpadKey>, ()> {
        match key {
            DirpadKey::A => return self.controls.input(self.key),
            DirpadKey::Up => self.key = self.key.up().ok_or(())?,
            DirpadKey::Right => self.key = self.key.right().ok_or(())?,
            DirpadKey::Down => self.key = self.key.down().ok_or(())?,
            DirpadKey::Left => self.key = self.key.left().ok_or(())?,
        };
        Ok(None)
    }
}
impl Robot for NumpadRobot {
    fn input(&mut self, key: DirpadKey) -> Result<Option<NumpadKey>, ()> {
        match key {
            DirpadKey::A => return Ok(Some(self.key)),
            DirpadKey::Up => self.key = self.key.up().ok_or(())?,
            DirpadKey::Right => self.key = self.key.right().ok_or(())?,
            DirpadKey::Down => self.key = self.key.down().ok_or(())?,
            DirpadKey::Left => self.key = self.key.left().ok_or(())?,
        };
        Ok(None)
    }
}
impl NumpadRobot {
    fn new(key: NumpadKey) -> NumpadRobot {
        NumpadRobot { key: NumpadKey::A }
    }
}
impl<NextBot: Robot + Clone> DirpadRobot<NextBot> {
    fn new(key: DirpadKey, controls: NextBot) -> DirpadRobot<NextBot> {
        DirpadRobot { key, controls }
    }
}
#[test]
fn test_chained_robots() {
    let a = NumpadRobot::new(NumpadKey::A);
    let b = DirpadRobot::new(DirpadKey::A, a);
    let mut c = DirpadRobot::new(DirpadKey::A, b);

    // A A [A]
    assert_eq!(c.input(DirpadKey::Left), Ok(None));
    assert_eq!(c.input(DirpadKey::Down), Ok(None));
    // v A [A]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // v > [A]
    assert_eq!(c.input(DirpadKey::Left), Ok(None));
    // < > [A]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // < < [A]
    assert_eq!(c.input(DirpadKey::Right), Ok(None));
    assert_eq!(c.input(DirpadKey::Right), Ok(None));
    assert_eq!(c.input(DirpadKey::Up), Ok(None));
    // A < [A]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // A < [0]
    assert_eq!(c.input(DirpadKey::Down), Ok(None));
    // > < [0]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // > v [0]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // > > [0]
    assert_eq!(c.input(DirpadKey::Up), Ok(None));
    assert_eq!(c.input(DirpadKey::Left), Ok(None));
    // ^ > [0]
    assert_eq!(c.input(DirpadKey::A), Ok(None));
    // ^ A [0]
    assert_eq!(c.input(DirpadKey::Right), Ok(None));
    // A A [0]
    assert_eq!(c.input(DirpadKey::A), Ok(Some(NumpadKey::Zero)));
    // A A [0] -> "0"
}
