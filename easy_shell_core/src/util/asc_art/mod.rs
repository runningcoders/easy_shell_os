use std::fmt::{Result, Write};

mod animal;

trait Printer {
    fn print(&self, writer: &mut dyn Write) -> Result;
    fn to_string(&self) -> String {
        let mut s = String::new();
        match self.print(&mut s) {
            Ok(_) => s,
            Err(e) => e.to_string(),
        }
    }
}
