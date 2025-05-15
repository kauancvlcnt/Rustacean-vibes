pub struct List {
    pub i_list: Vec<i64>,
    pub accumulator: i64,
    pub list_len: f64,
}

impl List {
    pub fn media(&self) -> f64 {
        self.accumulator as f64 / self.list_len
    }

    pub fn mediana(&self) -> f64 {
        let middle = self.list_len / 2 as f64;
        println!("{}", middle);
        let result = if middle % 2 as f64 != 0 as f64 {
            let floor = middle.floor();
            let round = middle.round();
            (floor + round) / 2 as f64
        } else {
            middle
        };
        result as f64
    }
}
