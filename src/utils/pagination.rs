pub struct Pagination {
    pub page: u64,
    pub size: u64,
}

impl Into<Pagination> for Option<u64> {
    fn into(self) -> Pagination {
        Pagination {
            page: self.unwrap_or(1),
            size: 20,
        }
    }
}

impl Pagination {
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn compute(&self) -> (u64, u64) {
        if self.page == 0 {
            return (0, self.size);
        }
        let offset = (self.page - 1) * self.size;
        (offset, self.size)
    }
}
