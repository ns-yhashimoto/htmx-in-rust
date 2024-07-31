pub mod root {
    use tera::{Context, Tera};
    pub fn render_index_page() -> String {
        let tera = Tera::new("src/view/templates/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/");
        tera.render("index.html", &ctx).unwrap()
    }
}

pub mod todo {
    use std::str::FromStr;

    use crate::model::todo::Todo;
    use chrono::{DateTime, Utc};
    use tera::{Context, Result, Tera, Value};

    pub fn render_index_page(todos: &Vec<Todo>) -> String {
        let mut tera = Tera::new("src/view/templates/**/*.html").unwrap();
        tera.register_tester("done", tester_done);

        let mut ctx = Context::new();
        ctx.insert("page", "/todo");
        ctx.insert("todos", todos);
        tera.render("todo/index.html", &ctx).unwrap()
    }

    pub fn render_items(todos: &Vec<Todo>) -> String {
        let mut tera = Tera::new("src/view/templates/**/*.html").unwrap();
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
        use crate::{model::todo, view};

        #[test]
        fn render_index_page_works() {
            let todos: Vec<todo::Todo> = vec![
                todo::Todo {
                    id: 1,
                    content: String::from("task 1"),
                    completed_on: None,
                },
                todo::Todo {
                    id: 2,
                    content: String::from("task 2"),
                    completed_on: Some(chrono::Utc::now()),
                },
            ];

            let _ = view::html::todo::render_index_page(&todos);
            assert!(true);
        }

        #[test]
        fn render_items_works() {
            let todos: Vec<todo::Todo> = vec![
                todo::Todo {
                    id: 1,
                    content: String::from("task 1"),
                    completed_on: None,
                },
                todo::Todo {
                    id: 2,
                    content: String::from("task 2"),
                    completed_on: Some(chrono::Utc::now()),
                },
            ];

            let _ = view::html::todo::render_items(&todos);
            assert!(true);
        }
    }
}

pub mod order {
    use crate::model::order::OrderBalance;
    use tera::{Context, Tera};
    pub fn render_index_page(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/order");
        ctx.insert("orders", orders);
        tera.render("order/index.html", &ctx).unwrap()
    }

    pub fn render_order_rows(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new("src/view/templates/**/*.html").unwrap();

        let mut ctx = Context::new();
        ctx.insert("orders", orders);
        tera.render("order/rows.html", &ctx).unwrap()
    }
}
