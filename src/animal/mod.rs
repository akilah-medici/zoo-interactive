#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animal {
    pub name: String,
    pub description: String,
    pub date_of_birth: String,
    pub spices: String,
    pub habitat: String,
    pub country_of_origin: String,
}

impl Animal {
    pub fn new<N, D, DB, S, H, C>(nm: N, desc: D, db: DB, s: S, h: H, c: C) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        DB: Into<String>,
        S: Into<String>,
        H: Into<String>,
        C: Into<String>,
    {
        Self {
            name: nm.into(),
            description: desc.into(),
            date_of_birth: db.into(),
            spices: s.into(),
            habitat: h.into(),
            country_of_origin: c.into(),
        }
    }
}
