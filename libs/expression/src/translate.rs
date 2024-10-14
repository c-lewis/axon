use polars::prelude::LazyFrame;

mod to_polars;

pub trait Translate<T> {
    fn translate(&self, data_provider: Box<dyn DataProvider>) -> Option<T>;
}

pub trait DataProvider {
    fn retrieve_feature_data(&self, feature_name: &str) -> Option<LazyFrame>;
}
