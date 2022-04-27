use crate::asc_art::Printer;
use std::fmt::{Result, Write};

enum Animal {
    DogHeadRight,
    DogHeadLeft,
}

impl Animal {
    pub fn tmpl_str(&self) -> &str {
        match self {
            Animal::DogHeadRight => DOG_HEAD_RIGHT,
            Animal::DogHeadLeft => DOG_HEAD_LEFT,
        }
    }
}

impl Printer for Animal {
    fn print(&self, writer: &mut dyn Write) -> Result {
        writer.write_str(self.tmpl_str())
    }
}

const DOG_HEAD_RIGHT: &str = r#"
         (\
        (\_\_^__o
 ___     `-'/ `_/
'`--\______/  |
    /         |
  -`/.------'\^-'
"#;

const DOG_HEAD_LEFT: &str = r#"
      /)
o__^^/_/)
 \ ' \`-'     ___
  `|  \______/--'`
   |         \
 ././-------,.\
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_len() {}
}
