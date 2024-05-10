use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::de;

// A Recipe represents a recipe for a meal kit
// from the marketplace.
#[derive(Deserialize, Debug, Clone)]
pub struct Recipe {

    // The recipe ID
    #[serde(rename = "recipe_id")]
    pub id: String,

    // The number of portions the recipe produces
    #[serde(
        rename = "portions",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub portions: usize,

    // The base total price of the recipe
    #[serde(rename = "price", deserialize_with = "de::deserialize_money")]
    pub price: Money<'static, iso::Currency>,

    // The base price per portion of the recipe
    #[serde(
        rename = "price_per_portion",
        deserialize_with = "de::deserialize_money"
    )]
    pub price_per_portion: Money<'static, iso::Currency>,

    // The active price per portion of the recipe
    #[serde(
        rename = "current_price_per_portion",
        deserialize_with = "de::deserialize_money"
    )]
    pub current_price_per_portion: Money<'static, iso::Currency>,

    // The active total price of the recipe
    #[serde(
        rename = "current_price",
        deserialize_with = "de::deserialize_money"
    )]
    pub current_price: Money<'static, iso::Currency>,

    // A list of ingredients in the recipe
    #[serde(rename = "ingredients")]
    pub ingredients: Vec<Ingredient>,
}

// An Ingredient ...todo
#[derive(Deserialize, Debug, Clone)]
pub struct Ingredient {
    #[serde(rename = "options")]
    pub options: Vec<IngredientOption>,
}

// An IngredientOption ...todo
#[derive(Deserialize, Debug, Clone)]
pub struct IngredientOption {}
