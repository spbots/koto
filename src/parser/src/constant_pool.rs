use {
    crate::ConstantIndex,
    std::{
        collections::{hash_map::DefaultHasher, HashMap},
        fmt,
        hash::{Hash, Hasher},
        ops::Range,
    },
};

#[derive(Clone, Debug, Hash, PartialEq)]
enum ConstantInfo {
    Number(usize),
    Str(Range<usize>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constant<'a> {
    Number(f64),
    Str(&'a str),
}

#[derive(Clone, Debug)]
pub struct ConstantPool {
    index: Vec<ConstantInfo>,
    // Constant strings concatanated into one
    strings: String,
    numbers: Vec<f64>,
    hash: u64,
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self {
            index: vec![],
            strings: String::default(),
            numbers: vec![],
            hash: 0,
        }
    }
}

impl ConstantPool {
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: ConstantIndex) -> Option<Constant> {
        match self.index.get(index as usize) {
            Some(constant_info) => match constant_info {
                ConstantInfo::Number(index) => Some(Constant::Number(self.numbers[*index])),
                ConstantInfo::Str(bounds) => Some(Constant::Str(&self.strings[bounds.clone()])),
            },
            None => None,
        }
    }

    pub fn string_data(&self) -> &str {
        &self.strings
    }

    #[inline]
    pub fn get_str(&self, index: ConstantIndex) -> &str {
        // Safety: The bounds have already been checked while the pool is being prepared
        unsafe { &self.strings.get_unchecked(self.get_str_bounds(index)) }
    }

    pub fn get_str_bounds(&self, index: ConstantIndex) -> Range<usize> {
        match self.index.get(index as usize) {
            Some(ConstantInfo::Str(bounds)) => bounds.clone(),
            _ => panic!("Invalid index"),
        }
    }

    pub fn get_number(&self, index: ConstantIndex) -> f64 {
        match self.index.get(index as usize) {
            Some(ConstantInfo::Number(index)) => self.numbers[*index],
            _ => panic!("Invalid index"),
        }
    }

    pub fn iter(&self) -> ConstantPoolIterator {
        ConstantPoolIterator::new(self)
    }
}

pub struct ConstantPoolIterator<'a> {
    pool: &'a ConstantPool,
    index: ConstantIndex,
}

impl<'a> ConstantPoolIterator<'a> {
    fn new(pool: &'a ConstantPool) -> Self {
        Self { pool, index: 0 }
    }
}

impl<'a> Iterator for ConstantPoolIterator<'a> {
    type Item = Constant<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.pool.get(self.index);
        self.index += 1;
        result
    }
}

impl fmt::Display for ConstantPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, constant) in self.iter().enumerate() {
            write!(f, "{}\t", i)?;
            match constant {
                Constant::Number(n) => write!(f, "Number\t{}", n)?,
                Constant::Str(s) => write!(f, "String\t{}", s)?,
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for ConstantPool {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.strings == other.strings && self.numbers == other.numbers
    }
}

impl Hash for ConstantPool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConstantPoolBuilder {
    pool: ConstantPool,
    hasher: DefaultHasher, // Used to incrementally hash the constant pool's contents
    string_map: HashMap<String, ConstantIndex>,
    number_map: HashMap<[u8; 8], ConstantIndex>,
}

impl ConstantPoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_string(&mut self, s: &str) -> ConstantIndex {
        match self.string_map.get(s) {
            Some(index) => *index,
            None => {
                let start = self.pool.strings.len();
                let end = start + s.len();
                self.pool.strings.push_str(s);
                s.hash(&mut self.hasher);

                let result = self.pool.index.len() as ConstantIndex;
                self.pool.index.push(ConstantInfo::Str(start..end));

                self.string_map.insert(s.to_string(), result);

                result
            }
        }
    }

    pub fn add_number(&mut self, n: f64) -> ConstantIndex {
        let bytes = n.to_ne_bytes();

        match self.number_map.get(&bytes) {
            Some(index) => *index,
            None => {
                let number_index = self.pool.numbers.len();
                self.pool.numbers.push(n);
                bytes.hash(&mut self.hasher);

                let result = self.pool.index.len() as ConstantIndex;
                self.pool.index.push(ConstantInfo::Number(number_index));

                self.number_map.insert(bytes, result);

                result
            }
        }
    }

    pub fn pool(&self) -> &ConstantPool {
        &self.pool
    }

    pub fn build(mut self) -> ConstantPool {
        self.pool.index.hash(&mut self.hasher);
        self.pool.hash = self.hasher.finish();
        self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats_are_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_adding_strings() {
        let mut builder = ConstantPoolBuilder::new();

        let s1 = "test";
        let s2 = "test2";

        // 1 byte for string length
        assert_eq!(0, builder.add_string(s1));
        assert_eq!(1, builder.add_string(s2));

        // don't duplicate strings
        assert_eq!(0, builder.add_string(s1));
        assert_eq!(1, builder.add_string(s2));

        let pool = builder.build();

        assert_eq!(s1, pool.get_str(0));
        assert_eq!(s2, pool.get_str(1));

        assert_eq!(2, pool.len());
    }

    #[test]
    fn test_adding_numbers() {
        let mut builder = ConstantPoolBuilder::new();

        let f1 = 1.23456789;
        let f2 = 9.87654321;

        assert_eq!(0, builder.add_number(f1));
        assert_eq!(1, builder.add_number(f2));

        // don't duplicate numbers
        assert_eq!(0, builder.add_number(f1));
        assert_eq!(1, builder.add_number(f2));

        let pool = builder.build();

        assert!(floats_are_equal(f1, pool.get_number(0)));
        assert!(floats_are_equal(f2, pool.get_number(1)));

        assert_eq!(2, pool.len());
    }

    #[test]
    fn test_adding_mixed_types() {
        let mut builder = ConstantPoolBuilder::new();

        let f1 = -1.1;
        let f2 = 99.9;
        let s1 = "O_o";
        let s2 = "^_^";

        assert_eq!(0, builder.add_number(f1));
        assert_eq!(1, builder.add_string(s1));
        assert_eq!(2, builder.add_number(f2));
        assert_eq!(3, builder.add_string(s2));

        let pool = builder.build();

        assert!(floats_are_equal(f1, pool.get_number(0)));
        assert!(floats_are_equal(f2, pool.get_number(2)));
        assert_eq!(s1, pool.get_str(1));
        assert_eq!(s2, pool.get_str(3));

        assert_eq!(4, pool.len());
    }

    #[test]
    fn test_iter() {
        let mut builder = ConstantPoolBuilder::new();

        let f1 = -1.1;
        let f2 = 99.9;
        let s1 = "O_o";
        let s2 = "^_^";

        builder.add_number(f1);
        builder.add_string(s1);
        builder.add_number(f2);
        builder.add_string(s2);

        let pool = builder.build();

        let mut iter = pool.iter();
        assert_eq!(iter.next(), Some(Constant::Number(-1.1)));
        assert_eq!(iter.next(), Some(Constant::Str("O_o")));
        assert_eq!(iter.next(), Some(Constant::Number(99.9)));
        assert_eq!(iter.next(), Some(Constant::Str("^_^")));
        assert_eq!(iter.next(), None);
    }
}
