use tera::{Context, Tera};

pub fn render_index_page(tera: &Tera) -> String {
    let ctx = Context::new();
    tera.render("index.html", &ctx).unwrap()
}