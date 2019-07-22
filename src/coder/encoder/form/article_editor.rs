use crate::entity::form::{
    article_editor::{Field, ValidForm as EntityValidForm},
    FormField,
};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ValidForm<'a> {
    article: IndexMap<&'a str, ValidFormValue<'a>>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ValidFormValue<'a> {
    Text(&'a str),
    Vector(Vec<&'a str>),
}

impl<'a> ValidForm<'a> {
    pub fn new(form: &'a EntityValidForm) -> Self {
        ValidForm {
            article: form
                .iter()
                .map(|(key, field)| match field {
                    Field::Tags(tags) => {
                        ("tagList", ValidFormValue::Vector(tags.split(' ').collect()))
                    }
                    _ => (*key, ValidFormValue::Text(field.value())),
                })
                .collect(),
        }
    }
}
