use std::{fmt, sync::Arc};

#[cfg(test)]
use proptest::prelude::Strategy;
#[cfg(test)]
use proptest::prop_compose;
#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Clone, Debug)]
pub struct DataSource {
    pub(crate) feature_name: String,
    pub(crate) exp: Arc<Expression>,
}

impl DataSource {
    pub fn new(feature_name: &str, exp: Expression) -> Self {
        Self {
            feature_name: feature_name.to_owned(),
            exp: Arc::new(exp),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FilterValue(Filter, Arc<Option<Expression>>),
    // GroupTimestamp(Grouping, Arc<Option<Expression>>),
    // Aggregate(Aggregation, Arc<Option<Expression>>),
}

impl Expression {
    pub fn next(&self) -> Arc<Option<Expression>> {
        use Expression::*;
        let exp = match self {
            FilterValue(_, exp) => exp,
            // GroupTimestamp(_, exp) => exp,
            // Aggregate(_, exp) => exp,
        };
        exp.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Filter {
    pub(crate) column: KnownColumn,
    pub(crate) op: Operator,
    pub(crate) value: Const,
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.column, self.op, self.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Grouping {
    ByFilter(Filter),
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Aggregation {
    Sum,
}

impl fmt::Display for Aggregation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Aggregation::*;
        let operation = match self {
            Sum => "sum",
        };
        write!(f, "{operation}")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum KnownColumn {
    KeyColumn,
    TimestampColumn,
    ValueColumn,
}

impl fmt::Display for KnownColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use KnownColumn::*;
        let name = match self {
            KeyColumn => "key",
            TimestampColumn => "timestamp",
            ValueColumn => "value",
        };
        write!(f, "{name}")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AxonDate {
    #[cfg_attr(test, proptest(strategy = "arb_date()"))]
    date: chrono::NaiveDate,
}

#[cfg(test)]
prop_compose! {
    pub fn arb_date()(date in (
            (1990..=2110i32),
            (1..=12u32),
            (1..=31u32)
        ).prop_filter_map("Invalid date", |(y, m, d)| chrono::NaiveDate::from_ymd_opt(y, m, d))
    ) -> chrono::NaiveDate {
        date
    }
}

impl Into<chrono::NaiveDate> for AxonDate {
    fn into(self) -> chrono::NaiveDate {
        self.date
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AxonTime {
    #[cfg_attr(test, proptest(strategy = "arb_time()"))]
    time: chrono::NaiveTime,
}

#[cfg(test)]
prop_compose! {
    pub fn arb_time()(time in (
            (1..=24u32),
            (1..=59u32),
            (1..=59u32)
        ).prop_filter_map("Invalid time", |(h, m, s)| chrono::NaiveTime::from_hms_opt(h, m, s))
    ) -> chrono::NaiveTime {
        time
    }
}

impl Into<chrono::NaiveTime> for AxonTime {
    fn into(self) -> chrono::NaiveTime {
        self.time
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AxonTimestamp(AxonDate, AxonTime);

impl Into<chrono::NaiveDateTime> for AxonTimestamp {
    fn into(self) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::new(self.0.into(), self.1.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Const {
    Null,
    Boolean(bool),
    String(String),
    Uint32(u32),
    Int32(i32),
    Float32(f32),
    Timestamp(AxonTimestamp),
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Const::*;
        let lit_type = match self {
            Null => "null",
            Boolean(_) => "boolean",
            String(_) => "string",
            Uint32(_) => "uint32",
            Int32(_) => "int32",
            Float32(_) => "float32",
            Timestamp(_) => "timestamp",
        };
        write!(f, "{lit_type}")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Operator {
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Operator::*;
        let symbol = match self {
            Eq => "==",
            NotEq => "!=",
            Gt => ">",
            GtEq => ">=",
            Lt => "<",
            LtEq => "<=",
        };
        write!(f, "{symbol}")
    }
}
