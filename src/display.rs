pub trait Display: Drop {
    fn getch(&self) -> Option<char>;
}
