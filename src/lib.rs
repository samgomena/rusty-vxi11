struct VXI11 {
    link: Option<usize>,
    client: usize,
}

impl VXI11 {
    fn open(self) -> Option<bool> {
        if self.link.is_none() {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
