/// Factory simulator simulates a running factory.
/// Author: G0rocks
/// Date: 2025-04-30

// Import crates
// use geo::Point; // For representing geographical points
use std::{collections::HashMap}; // For representing a HashMap


// Structs
// -------------------------------------------------------

/// Struct representing a product
/// Products are what the factory produces, they need a recipe with ingredients which is what the factory needs to make the product
/// ### Warninggeo::Point::new(longitude, latitude)
/// The recipe is a Hashmap with the ingredients as keys and their amounts as values but no units so you must make sure that your units are correct.
/// This means that if you want to make a product that is 1kg of bread, then your hashmap might be {"flour": 0.5, "water": 0.3, "yeast": 0.2} but you have to know that the 1 bread is 1 kg of bread, not 1 loaf of bread and you need to know the units of your ingredients.
#[derive(Debug)]
// #[derive(Display)]
pub struct Product {
    /// The product name
    pub name: String,
    /// The recipe for the product, keys signify ingredients and values signify the amount of the ingredient needed to make 1 unit of the product
    pub recipe: HashMap<String, f64>,
}

/// Struct representing a factory
/// All parameters are wrapped in an Optional type so you can have a factory with some unknown parameters
#[derive(Debug)]
pub struct Factory {
    /// Product that the factory produces
    pub product: Option<Product>,
    /// How much product the factory produces in a 24 hour time period
    pub production_capacity: Option<f64>,
    /// Note: The key in this hashmap is the same as the key in the recipe in [Product] but this time the value is the amount the factory can store of that ingredient.
    pub ingredient_storage_capacity: Option<HashMap<String, f64>>,
    /// Location of the factory
    pub location: Option<geo::Point>,
    /// How much ingredient the factory currently has in storage
    pub ingredient_available: Option<HashMap<String, f64>>,
}

/// Struct representing a delivery of an ingredient to the factory
#[derive(Debug)]
pub struct Delivery {
    /// The name of the ingredient
    pub name: String,
    /// The amount of the ingredient
    pub amount: f64,
    /// The time of delivery ingredient, in days
    pub delivery_time: f64,
}

// Implementations for Factory
// -------------------------------------------------------
impl Factory {
    /// Creates a new factory with default values
    pub fn new() -> Factory {
        Factory {
            product: None,
            production_capacity: None,
            ingredient_storage_capacity: None,
            location: None,
            ingredient_available: None,
        }
    }

    /// Evaluates the supply chain for a given product
    pub fn evaluate_supply_chain(&self, product: Product, _deliveries: Vec<Delivery>) -> Vec<f64> {
        // Init empty ingredient storage status vector
        let mut _ingredient_storage_status: Vec<f64> = vec![];

        // Loop through each delivery, log the storage status at the time of delivery
        // for delivery in deliveries.iter() {
            // Check if the ingredient is in the recipe for the product
        // }

        // Return the ingredient storage status vector
        todo!("Implement supply chain evaluation for ingredient: {}", product.name);
        // return ingredient_storage_status;
    }

}


// Implementations for Product
// -------------------------------------------------------
impl Product {
    /// Creates a new product
    pub fn new(n: String, r: HashMap<String, f64>) -> Product {
        Product {
            name: n,
            recipe: r,
        }
    }
}


// Tests for the library
//-------------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
