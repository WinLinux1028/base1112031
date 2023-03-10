pub trait FromReverseVecChar {
    fn from(from: Vec<char>) -> Self;
}
impl FromReverseVecChar for Vec<char> {
    fn from(mut from: Vec<char>) -> Self {
        from.reverse();
        from
    }
}
impl FromReverseVecChar for String {
    fn from(from: Vec<char>) -> Self {
        from.into_iter().rev().collect()
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
