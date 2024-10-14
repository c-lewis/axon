use std::{borrow::BorrowMut, ops::Deref, sync::Arc};

use chrono::NaiveDateTime;
use polars::prelude::{col, lit, Expr, LazyFrame, NULL};

use super::{DataProvider, Translate};
use crate::expression::{Const, DataSource, Expression, Filter, Operator};

struct TranslationState {
    lazy_frame: LazyFrame,
}

/// Translate an `Expression` into a Polars `LazyFrame`
impl Translate<LazyFrame> for DataSource {
    fn translate(&self, data_provider: Box<dyn DataProvider>) -> Option<LazyFrame> {
        let Some(lazy_frame) = data_provider.retrieve_feature_data(&self.feature_name) else {
            return None;
        };
        translate(
            TranslationState { lazy_frame }.borrow_mut(),
            data_provider,
            self.exp.clone(),
        )
    }
}

fn translate(
    state: &mut TranslationState,
    data_provider: Box<dyn DataProvider>,
    exp: Arc<Expression>,
) -> Option<LazyFrame> {
    use Expression::*;
    let lazy_frame = match exp.deref() {
        FilterValue(filter, _) => {
            let filter = translate_filter(&filter);
            state.lazy_frame.clone().filter(filter)
        } // GroupTimestamp(grouping, next) => translate_grouping(state, grouping),
          // Aggregate(aggregation, next) => translate_aggregation(state, aggregation),
    };
    match *exp.next() {
        Some(_) => {
            state.lazy_frame = lazy_frame;
            translate(state, data_provider, exp)
        }
        None => Some(lazy_frame),
    }
}

fn translate_filter(filter: &Filter) -> Expr {
    let column = col(format!("{}", filter.column));
    let value = translate_const(&filter.value);
    use Operator::*;
    match filter.op {
        Eq => column.eq(value),
        NotEq => column.neq(value),
        Gt => column.gt(value),
        GtEq => column.gt_eq(value),
        Lt => column.lt(value),
        LtEq => column.lt_eq(value),
    }
}

fn translate_const(value: &Const) -> Expr {
    match value {
        Const::Null => lit(NULL),
        Const::Boolean(x) => lit(*x),
        Const::String(x) => lit(x.clone()),
        Const::Uint32(x) => lit(*x),
        Const::Int32(x) => lit(*x),
        Const::Float32(x) => lit(*x),
        Const::Timestamp(x) => lit::<NaiveDateTime>(x.to_owned().into()),
    }
}

// fn translate_filter(state: &mut TranslationState, grouping: &Grouping) -> LazyFrame {}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::expression::AxonTimestamp;

    use super::*;

    #[test]
    fn translate_null_const_to_exp() {
        let result = translate_const(&Const::Null);
        assert_eq!(result, lit(NULL));
    }

    proptest! {
        #[test]
        fn translate_bool_const_to_exp(value: bool) {
            let result = translate_const(&Const::Boolean(value));
            assert_eq!(result, lit(value));
        }

        #[test]
        fn translate_string_const_to_exp(value: String) {
            let result = translate_const(&Const::String(value.clone()));
            assert_eq!(result, lit(value.clone()));
        }

        #[test]
        fn translate_uint32_const_to_exp(value: u32) {
            let result = translate_const(&Const::Uint32(value));
            assert_eq!(result, lit(value));
        }

        #[test]
        fn translate_int32_const_to_exp(value: i32) {
            let result = translate_const(&Const::Int32(value));
            assert_eq!(result, lit(value));
        }

        #[test]
        fn translate_float32_const_to_exp(value: f32) {
            let result = translate_const(&Const::Float32(value));
            assert_eq!(result, lit(value));
        }

        #[test]
        fn translate_timestamp_const_to_exp(value: AxonTimestamp) {
            let result = translate_const(&Const::Timestamp(value.clone()));
            assert_eq!(result, lit::<NaiveDateTime>(value.into()));
        }

        #[test]
        fn translate_filter_to_exp(filter: Filter) {
            let column = col(format!("{}", filter.column));
            use Operator::*;
            let expected_exp = match filter.op {
                Eq => column.eq(translate_const(&filter.value)),
                NotEq => column.neq(translate_const(&filter.value)),
                Gt => column.gt(translate_const(&filter.value)),
                GtEq => column.gt_eq(translate_const(&filter.value)),
                Lt => column.lt(translate_const(&filter.value)),
                LtEq => column.lt_eq(translate_const(&filter.value)),
            };
            let result = translate_filter(&filter);
            assert_eq!(result, expected_exp);
        }
    }
}
