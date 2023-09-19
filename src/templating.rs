extern crate lazy_static;
use tera::Tera;
use serde_json::Value;
use std::fs::{File, self, ReadDir};
use std::io::Read;
use std::error;
use lazy_static::lazy_static;

lazy_static! {
    static ref TERA: Tera = {
        let mut tera = Tera::new("Static/Template/*").unwrap();
        tera
    };
}

pub enum templating_error {
    TemplateError(String),
    DataError(String),
}

pub fn templating(template: &str, data: Value) -> String {
    let mut tera = Tera::new("Static/Template/*").unwrap();
    let context = tera::Context::from_value(data).unwrap();
    let rendered = tera.render(&template, &context).unwrap();
    return rendered
}

pub fn template_single_val(template: &str, name: &str, value: String) -> String {
    let mut tera = Tera::new("Static/Template/*").unwrap();
    let mut context = tera::Context::new();
    context.insert(name, &value);
    let rendered = tera.render(&template, &context).unwrap();
    return rendered
}

pub fn get_json_by_file(path: &str) -> Value {
    let mut file = File::open(path).unwrap();
    let mut file_str: String = String::new();
    file.read_to_string(&mut file_str);
    let json_data: &str = &file_str;
    let parsed_data = serde_json::from_str(json_data).unwrap();
    return parsed_data
}