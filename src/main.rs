mod utils;
mod filter;

use std::collections::HashMap;

fn main() {
    let mut products:HashMap<String, f32> = HashMap::new();
    add_products(&mut products);
 
    println!("Enter a minimum price");
    let min = utils::get_price();
    println!("Enter a maximun price");
    let max = utils::get_price();

    println!("{:.2} < price < {:.2}", min, max);

    let list = filter::filter(&products, min, max);

    println!("Products between {:.2} - {:.2}:", min, max);
    for product in list {
        println!("{}", product);
    }
}


fn add_products(products:&mut HashMap<String, f32>) {
    products.insert("Organic Bananas (1 lb)".to_string(), 0.59);
    products.insert("Whole Wheat Bread".to_string(), 2.49);
    products.insert("Fresh Avocados (each)".to_string(), 1.99);
    products.insert("Organic Chicken Breasts (1 lb)".to_string(), 6.99);
    products.insert("Fresh Salmon (1 lb)".to_string(), 12.99);
    products.insert("Coca-Cola 12-pack".to_string(), 5.99);
    products.insert("Nestle Bottled Water (24 pack)".to_string(), 4.49);
    products.insert("Large Eggs (1 dozen)".to_string(), 3.19);
    products.insert("Organic Baby Spinach (5 oz)".to_string(), 3.99);
    products.insert("Cheddar Cheese (8 oz block)".to_string(), 4.29);
    products.insert("Skim Milk (1 gallon)".to_string(), 3.69);
    products.insert("Coca-Cola 2L".to_string(), 1.99);
    products.insert("Peanut Butter (16 oz jar)".to_string(), 2.89);
    products.insert("Toilet Paper (12-pack)".to_string(), 6.99);
    products.insert("Dish Soap (16 oz)".to_string(), 2.39);
    products.insert("Tomato Sauce (24 oz can)".to_string(), 1.29);
    products.insert("Frozen Pizza (12-inch)".to_string(), 6.99);
    products.insert("Granola Bars (box of 6)".to_string(), 3.99);
    products.insert("Canned Tuna (5 oz can)".to_string(), 1.09);
    products.insert("Carrots (1 lb)".to_string(), 1.49);
    products.insert("Russet Potatoes (5 lb bag)".to_string(), 3.99);
    products.insert("Orange Juice (64 oz)".to_string(), 3.49);
    products.insert("Butter (1 lb)".to_string(), 4.29);
    products.insert("Bag of Ice (10 lbs)".to_string(), 2.99);
}