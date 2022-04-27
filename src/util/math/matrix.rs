use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(PartialEq, Clone, Default, Debug)]
struct Matrix<T>
where
    T: Default + Clone,
{
    lines: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn new(lines: Vec<Vec<T>>) -> Self {
        let mut m = Matrix { lines };
        m.fix_size(T::default());
        m
    }

    pub fn with_size(x_len: usize, y_len: usize) -> Self {
        Matrix {
            lines: vec![vec![T::default(); x_len]; y_len],
        }
    }

    pub fn with_size_v(x_len: usize, y_len: usize, v: T) -> Self {
        Matrix {
            lines: vec![vec![v; x_len]; y_len],
        }
    }

    pub fn y_len(&self) -> usize {
        self.lines.len()
    }

    pub fn x_len(&self) -> usize {
        if self.y_len() == 0 {
            0
        } else {
            self.lines[0].len()
        }
    }

    pub fn is_safe_y(&self, y: usize) -> bool {
        y < self.y_len()
    }

    pub fn is_safe_x(&self, x: usize) -> bool {
        x < self.x_len()
    }

    pub fn exist(&self, x: usize, y: usize) -> bool {
        self.is_safe_y(y) && self.is_safe_x(x)
    }

    pub fn get_y(&self, y: usize) -> Option<&Vec<T>> {
        if self.is_safe_y(y) {
            self.lines.get(y)
        } else {
            None
        }
    }

    pub fn get_mut_y(&mut self, y: usize) -> Option<&mut Vec<T>> {
        if self.is_safe_y(y) {
            self.lines.get_mut(y)
        } else {
            None
        }
    }

    pub fn get_xy(&self, x: usize, y: usize) -> Option<&T> {
        if self.exist(x, y) {
            self.lines.get(y).unwrap().get(x)
        } else {
            None
        }
    }

    pub fn get_mut_xy(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.exist(x, y) {
            self.lines.get_mut(y).unwrap().get_mut(x)
        } else {
            None
        }
    }

    pub fn set_xy(&mut self, x: usize, y: usize, v: T) {
        if self.exist(x, y) {
            self.lines[y][x] = v
        }
    }

    pub fn fix_size(&mut self, v: T) {
        let mut x_len = 0;
        for x in self.lines.iter() {
            x_len = x_len.max(x.len());
        }
        self.expand_size(x_len, self.y_len(), v)
    }

    pub fn expand_size(&mut self, x_len: usize, y_len: usize, v: T) {
        for line in self.lines.iter_mut() {
            if line.len() < x_len {
                line.append(&mut vec![v.clone(); x_len - line.len()]);
            }
        }

        if self.y_len() < y_len {
            self.lines.push(vec![v; x_len]);
        }
    }
}

impl FromStr for Matrix<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_len = 0;
        let mut ls = vec![];
        for line in s.lines() {
            let line = line.chars();
            let mut l = Vec::with_capacity(line.clone().count());
            for v in line {
                l.push(v);
            }

            x_len = x_len.max(l.len());
            ls.push(l);
        }

        Ok(Self::new(ls))
    }
}

impl<T> Display for Matrix<T>
where
    T: Default + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter() {
            let line: Vec<String> = line.iter().map(|v| v.to_string()).collect();
            f.write_str(&line.join("\t"))?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_from_str() {
        let s = r#"
    test
1111
"#;
        let a = Matrix::from_str(&s[1..s.len() - 1]).unwrap();
        let b = Matrix::new(vec![
            vec![' ', ' ', ' ', ' ', 't', 'e', 's', 't'],
            vec!['1', '1', '1', '1'],
        ]);
        assert_eq!(a, b)
    }

    #[test]
    fn matrix_print() {
        println!(
            "{}",
            Matrix::new(vec![
                vec![' ', ' ', ' ', ' ', 't', 'e', 's', 't'],
                vec!['1', '1', '1', '1'],
            ])
        )
    }
}
