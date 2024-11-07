use std::collections::HashMap;

pub fn filter(products:&HashMap<String, f32>, min:f32, max:f32) -> Vec<String> {
    let mut filtered_products:Vec<String> = Vec::new();

    for (key,&value) in products {
        if value >= min && value <= max {
            let product_info = format!("[{}:{:.2}]", key, value);
            filtered_products.push(product_info);
        }
    }

    return filtered_products;
}