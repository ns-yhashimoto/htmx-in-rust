pub fn get_template_path(target: &str) -> String {
    let cd = std::env::current_dir().unwrap();
    let base_path = if cd.ends_with("order") {
        "templates"
    } else {
        "order/templates"
    };
    format!("{}/{}", base_path, target)
}

pub mod html {
    use tera::{Context, Tera};

    use crate::model::OrderBalance;
    use super::get_template_path;

    pub fn render_index_page(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new(&&get_template_path("**/*.html")).unwrap();

        let mut ctx = Context::new();
        ctx.insert("page", "/order");
        ctx.insert("orders", orders);
        tera.render("order/index.html", &ctx).unwrap()
    }

    pub fn render_order_rows(orders: &Vec<OrderBalance>) -> String {
        let tera = Tera::new(&get_template_path("**/*.html")).unwrap();

        let mut ctx = Context::new();
        ctx.insert("orders", orders);
        tera.render("order/rows.html", &ctx).unwrap()
    }
}
