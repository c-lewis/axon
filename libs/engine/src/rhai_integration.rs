use rhai::{CustomType, TypeBuilder};

use expression::expression::{DataSource, Expression};

#[derive(Clone, Debug, CustomType)]
#[rhai_type(extra = register_rhai_api)]
pub struct DataSourceBuilder {
    pub(crate) feature_name: Option<String>,
    pub(crate) exp: Option<Expression>,
}

impl DataSourceBuilder {
    pub fn new(feature_name: &str) -> Self {
        DataSourceBuilder {
            feature_name: Some(feature_name.to_owned()),
            exp: None,
        }
    }

    pub fn build(&self) -> Option<DataSource> {
        let feature_name = self.feature_name.clone()?;
        let expression = self.exp.clone()?;
        DataSource::new(&feature_name, expression).into()
    }
}

pub fn register_rhai_api(builder: &mut TypeBuilder<DataSourceBuilder>) {
    builder.with_fn("load_data", DataSourceBuilder::new);
}
