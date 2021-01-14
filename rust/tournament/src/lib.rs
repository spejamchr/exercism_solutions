struct Record {
    name: String,
    w: usize,
    d: usize,
    l: usize,
}

enum GameResult {
    Win(String, String),
    Draw(String, String),
}

impl Record {
    pub fn new(name: &str) -> Self {
        Self::make(name, 0, 0, 0)
    }

    fn make<N: ToString>(name: N, w: usize, l: usize, d: usize) -> Self {
        Self {
            name: name.to_string(),
            w,
            l,
            d,
        }
    }

    fn add_win(&self) -> Self {
        Self::make(&self.name, self.w + 1, self.l, self.d)
    }

    fn add_loss(&self) -> Self {
        Self::make(&self.name, self.w, self.l + 1, self.d)
    }

    fn add_draw(&self) -> Self {
        Self::make(&self.name, self.w, self.l, self.d + 1)
    }

    pub fn add(&self, r: &GameResult) -> Self {
        match r {
            GameResult::Win(a, _) if a == &self.name => self.add_win(),
            GameResult::Win(_, b) if b == &self.name => self.add_loss(),
            GameResult::Draw(a, _) if a == &self.name => self.add_draw(),
            GameResult::Draw(_, b) if b == &self.name => self.add_draw(),
            _ => Self::make(&self.name, self.w, self.l, self.d),
        }
    }

    pub fn mp(&self) -> usize {
        self.w + self.l + self.d
    }

    pub fn p(&self) -> usize {
        3 * self.w + self.d
    }
}

struct GameRecords {
    records: Vec<Record>,
}

impl GameRecords {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    fn add_record_if_missing(mut self, name: &str) -> Self {
        if self.records.iter().find(|r| r.name == name).is_none() {
            self.records.push(Record::new(name));
        }
        self
    }

    pub fn add(self, r: &GameResult) -> Self {
        match r {
            GameResult::Draw(a, b) | GameResult::Win(a, b) => {
                let mut records = self
                    .add_record_if_missing(a)
                    .add_record_if_missing(b)
                    .records
                    .iter()
                    .map(|record| record.add(r))
                    .collect::<Vec<_>>();
                records.sort_by(|a, b| (b.p(), &a.name).cmp(&(a.p(), &b.name)));
                Self { records }
            }
        }
    }
}

impl std::fmt::Display for GameRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::iter::once(format!(
                "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                "Team", "MP", "W", "D", "L", "P"
            ))
            .chain(self.records.iter().map(|r| r.to_string()))
            .collect::<Vec<String>>()
            .join("\n")
        )
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.mp(),
            self.w,
            self.d,
            self.l,
            self.p()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    match_results
        .split('\n')
        .flat_map(|r| r.split(';'))
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| match c {
            [a, b, "win"] => Some(GameResult::Win((*a).to_string(), (*b).to_string())),
            [a, b, "loss"] => Some(GameResult::Win((*b).to_string(), (*a).to_string())),
            [a, b, "draw"] => Some(GameResult::Draw((*a).to_string(), (*b).to_string())),
            [..] => None,
        })
        .fold(GameRecords::new(), |records, option| match option {
            Some(gr) => records.add(&gr),
            None => records,
        })
        .to_string()
}
