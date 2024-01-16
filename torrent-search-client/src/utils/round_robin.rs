use std::ops::AddAssign;

use surf::Url;

pub struct RoundRobin {
    index: usize,
    urls: Vec<Url>,
}

impl RoundRobin {
    pub fn get(&mut self) -> Url {
        self.index.add_assign(1);
        let i = self.index % self.urls.len();
        self.urls[i].clone()
    }
    pub fn new(urls: Vec<Url>) -> Self {
        Self { index: 0, urls }
    }
}
