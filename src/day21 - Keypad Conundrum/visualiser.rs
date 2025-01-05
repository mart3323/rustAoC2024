use crate::simulator::{DirpadKey, DirpadRobot, NumpadKey, NumpadRobot, Robot};

pub trait PrintRecursive {
    fn print_self(&self) -> String;
    fn next(&self) -> Option<impl PrintRecursive>;
    fn print_recursive(&self) -> String {
        let string = self.print_self();
        let string2 = self.next().map_or(String::from(""), |n| n.print_recursive());
        let height1 = string.lines().count();
        let width1 = string.lines().map(|l| l.chars().count()).max().unwrap_or(0);
        let height2 = string2.lines().count();
        let width2 = string2.lines().map(|l| l.chars().count()).max().unwrap_or(0);


        let mut out = String::new();

        for y in (0..height1.max(height2)).rev() {
            // Print current item
            let index = height1 as isize - 1 - y as isize;
            if 0 <= index {
                let line = string.lines().nth(index as usize).unwrap_or("");
                out.push_str(line);
                out.push_str(&" ".repeat(width1 - line.chars().count()));
            } else {
                out.push_str(&" ".repeat(width1));
            }
            // Print rest
            let index = height2 as isize - 1 - y as isize;
            if 0 <= index {
                let line = string2.lines().nth(index as usize).unwrap_or("");
                out.push_str(line);
                // out.push_str(&"-".repeat(width2 - line.chars().count()));
            }
            out.push_str("\n");
        }
        return String::from(out.trim_end());
    }
}
impl PrintRecursive for NumpadRobot {
    fn print_self(&self) -> String {
        let template = "\
┌─┬─┬─┐
│7│8│9│
├─┼─┼─┤
│4│5│6│
├─┼─┼─┤
│1│2│3│
└─┼─┼─┤
  │0│A│
  └─┴─┘";
        let replace = ['0','1','2','3','4','5','6','7','8','9','0','A'];
        let except = match self.key {
            NumpadKey::A => 'A',
            NumpadKey::Zero => '0',
            NumpadKey::One => '1',
            NumpadKey::Two => '2',
            NumpadKey::Three => '3',
            NumpadKey::Four => '4',
            NumpadKey::Five => '5',
            NumpadKey::Six => '6',
            NumpadKey::Seven => '7',
            NumpadKey::Eight => '8',
            NumpadKey::Nine => '9',
        };
        template.chars().map(|c|
            if replace.contains(&c) && except != c {' '} else {c}
        ).collect()
    }
    fn next(&self) -> Option<impl PrintRecursive> {
        None::<NumpadRobot>
    }
}
impl<NextBot: PrintRecursive+Robot+Clone> PrintRecursive for DirpadRobot<NextBot> {
    fn print_self(&self) -> String {
        let template = String::from("\
. ┌─┬─┐
. │↑│A│
┌─┼─┼─┤
│←│↓│→│
└─┴─┴─┘");
        let replace = ['A','↓','←','↑','→', '.'];
        let except = match self.key {
            DirpadKey::A => 'A',
            DirpadKey::Up => '↑',
            DirpadKey::Left => '←',
            DirpadKey::Down => '↓',
            DirpadKey::Right => '→',
        };
        template.chars().map(|c|
            if replace.contains(&c) && c != except {' '} else {c}
        ).collect()
    }
    fn next(&self) -> Option<impl PrintRecursive> {
        Some(self.controls.clone())
    }
}

#[test]
fn test_print() {
    let r = DirpadRobot{key: DirpadKey::A, controls: DirpadRobot{ key: DirpadKey::Right, controls: NumpadRobot {key: NumpadKey::Eight}}};

    println!("{}", r.print_recursive())
}