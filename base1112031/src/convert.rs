pub trait FromVecChar {
    fn from_vec_char(from: Vec<char>) -> Self;
}
impl FromVecChar for Vec<char> {
    fn from_vec_char(from: Vec<char>) -> Self {
        from
    }
}
impl FromVecChar for String {
    fn from_vec_char(from: Vec<char>) -> Self {
        from.into_iter().collect()
    }
}

pub trait ToVecChar {
    fn to_vec_char(self) -> Vec<char>;
}
impl ToVecChar for Vec<char> {
    fn to_vec_char(self) -> Vec<char> {
        self
    }
}
impl ToVecChar for &[char] {
    fn to_vec_char(self) -> Vec<char> {
        self.to_vec()
    }
}
impl ToVecChar for &str {
    fn to_vec_char(self) -> Vec<char> {
        self.chars().collect()
    }
}
