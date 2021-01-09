use core::iter::once;

type Rows = Vec<Vec<u32>>;

pub struct PascalsTriangle {
    rows: Rows,
}

fn calc_rows(row_count: u32) -> Rows {
    if row_count == 0 {
        return vec![];
    };
    if row_count == 1 {
        return vec![vec![1]];
    };

    let mut rows = calc_rows(row_count - 1);
    let next = once(1)
        .chain(
            rows.iter()
                .last()
                .unwrap()
                .windows(2)
                .map(|a| a.iter().sum()),
        )
        .chain(once(1))
        .collect();

    rows.push(next);
    rows
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = calc_rows(row_count);
        Self { rows }
    }

    pub fn rows(&self) -> Rows {
        self.rows.clone()
    }
}
