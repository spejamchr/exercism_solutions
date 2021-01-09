#[derive(Debug, PartialEq)]
enum DnaBase {
    G,
    C,
    T,
    A,
}

#[derive(Debug, PartialEq)]
enum RnaBase {
    C,
    G,
    A,
    U,
}

fn dna_base(ch: char, i: usize) -> Result<DnaBase, usize> {
    match ch {
        'G' => Ok(DnaBase::G),
        'C' => Ok(DnaBase::C),
        'T' => Ok(DnaBase::T),
        'A' => Ok(DnaBase::A),
        _ => Err(i),
    }
}

fn rna_base(ch: char, i: usize) -> Result<RnaBase, usize> {
    match ch {
        'C' => Ok(RnaBase::C),
        'G' => Ok(RnaBase::G),
        'A' => Ok(RnaBase::A),
        'U' => Ok(RnaBase::U),
        _ => Err(i),
    }
}

fn dna_to_rna(dna: &DnaBase) -> RnaBase {
    match dna {
        DnaBase::G => RnaBase::C,
        DnaBase::C => RnaBase::G,
        DnaBase::T => RnaBase::A,
        DnaBase::A => RnaBase::U,
    }
}

fn push_into<L, R>(mut v: Vec<L>, r: Result<L, R>) -> Result<Vec<L>, R> {
    r.map(|t| {
        v.push(t);
        v
    })
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: Vec<DnaBase>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: Vec<RnaBase>,
}

fn build_from_string_into_drna<DR, F>(string: &str, f: F) -> Result<Vec<DR>, usize>
where
    F: Fn(char, usize) -> Result<DR, usize>,
{
    string
        .chars()
        .enumerate()
        .fold(Ok(Vec::new()), |rv, (i, c)| {
            rv.and_then(|v| push_into(v, f(c, i)))
        })
}

impl Dna {
    pub fn new(dna_string: &str) -> Result<Dna, usize> {
        build_from_string_into_drna(dna_string, dna_base).map(|dna| Self { dna })
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.dna.iter().map(|d| dna_to_rna(d)).collect();
        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna_string: &str) -> Result<Rna, usize> {
        build_from_string_into_drna(rna_string, rna_base).map(|rna| Self { rna })
    }
}
