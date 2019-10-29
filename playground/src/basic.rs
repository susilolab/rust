use std::collections::HashMap;
use tera::{Tera, Context, Result};
use serde_json::value::{Value, to_value};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

pub fn basic_templating() {
    let mut context = Context::new();
    context.insert("username", &"Agus");
    context.insert("numbers", &vec![1,2,3]);
    context.insert("show_all", &false);
    context.insert("bio", &"<script>alert('pwnd');</script>");

    Tera::one_off("hello", &Context::new(), true).unwrap();

    match TEMPLATES.render("users/profile.html", &context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
}