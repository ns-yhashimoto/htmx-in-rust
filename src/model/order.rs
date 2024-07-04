use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OrderBalance<'a> {
    order_id: u32,
    status: &'a str
}

pub fn get_order_balance_list() -> Vec<OrderBalance<'static>> {
    vec![
        OrderBalance {
            order_id: 3,
            status: "未回答"
        },
        OrderBalance {
            order_id: 2,
            status: "引き当て済み"
        },
        OrderBalance {
            order_id: 1,
            status: "引き当て済み"
        },
    ]
}
