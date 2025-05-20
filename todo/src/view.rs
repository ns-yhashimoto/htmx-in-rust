pub fn get_template_path(target: &str) -> String {
    let cd = std::env::current_dir().unwrap();
    let base_path = if cd.ends_with("todo") {
        "templates"
    } else {
        "todo/templates"
    };
    format!("{}/{}", base_path, target)
}

pub mod html {
    use tera::{Context, Result, Tera, Value};
    use std::str::FromStr;
    use chrono::{DateTime, Utc};

    use crate::model::Todo;
    use super::get_template_path;

    pub fn render_index_page(todos: &Vec<Todo>) -> String {
        let mut tera = Tera::new(&get_template_path("**/*.html")).unwrap();
        tera.register_tester("done", tester_done);

        let mut ctx = Context::new();
        ctx.insert("page", "/todo");
        ctx.insert("todos", todos);
        tera.render("todo/index.html", &ctx).unwrap()
    }

    pub fn render_items(todos: &Vec<Todo>) -> String {
        let mut tera = Tera::new(&get_template_path("**/*.html")).unwrap();
        tera.register_tester("done", tester_done);

        let mut ctx = Context::new();
        ctx.insert("todos", todos);
        tera.render("todo/items.html", &ctx).unwrap()
    }

    fn tester_done(value: Option<&Value>, _: &[Value]) -> Result<bool> {
        match value {
            Some(Value::String(dt)) => match DateTime::<Utc>::from_str(dt) {
                Ok(parsed) => Ok(Utc::now() >= parsed),
                _ => Ok(false),
            },
            _ => Ok(false),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::html;
        use crate::model;

        #[test]
        fn render_index_page_works() {
            let todos: Vec<model::Todo> = vec![
                model::Todo {
                    id: 1,
                    content: String::from("task 1"),
                    completed_on: None,
                },
                model::Todo {
                    id: 2,
                    content: String::from("task 2"),
                    completed_on: Some(chrono::Utc::now()),
                },
            ];

            let _ = html::render_index_page(&todos);
            assert!(true);
        }

        #[test]
        fn render_items_works() {
            let todos: Vec<model::Todo> = vec![
                model::Todo {
                    id: 1,
                    content: String::from("task 1"),
                    completed_on: None,
                },
                model::Todo {
                    id: 2,
                    content: String::from("task 2"),
                    completed_on: Some(chrono::Utc::now()),
                },
            ];

            let _ = html::render_items(&todos);
            assert!(true);
        }
    }
}
