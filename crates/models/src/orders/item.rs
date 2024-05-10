use std::collections::HashMap;

use rusty_money::{iso, Money};
use serde::Deserialize;
use serde_aux::prelude::*;

use crate::de;

// An OrderItem represents an item from the marketplace
// that has been added to the user's current order.
// It contains product information as well as order-specific
// information such as quantity and pricing details.
#[derive(Deserialize, Debug)]
pub struct OrderItem {

    // The product ID of the item
    #[serde(rename = "product_id")]
    pub product_id: String,

    // The name of the item
    #[serde(rename = "p_name")]
    pub name: String,

    // The vendor of the item
    #[serde(rename = "s_name")]
    pub vendor: String,

    // The description of the item.
    // Note: this is usually null for some reason.
    #[serde(default, rename = "description")]
    pub description: Option<String>,

    // Whether or not the product is on sale.
    // Note: this is usally null for some reason.
    #[serde(
        default,
        rename = "on_sale",
        deserialize_with = "de::deserialize_bool"
    )]
    pub on_sale: Option<bool>,

    // The category that the product belongs to
    #[serde(rename = "cat_na")]
    pub category: String,

    // The primary image URL for the product
    #[serde(rename = "image_url")]
    pub image_url: String,

    // Additional image URLs for the product
    // in different sizes
    #[serde(rename = "image_urls")]
    pub image_urls: HashMap<String, String>,

    // TODO -- not sure what this is
    #[serde(
        rename = "default_price",
        deserialize_with = "de::deserialize_money"
    )]
    pub default_price: Money<'static, iso::Currency>,

    // The product price that is shown on the marketplace
    #[serde(
        rename = "defined_price",
        deserialize_with = "de::deserialize_money"
    )]
    pub defined_price: Money<'static, iso::Currency>,

    // TODO -- not sure what this is
    #[serde(
        rename = "price",
        deserialize_with = "de::deserialize_money"
    )]
    pub price: Money<'static, iso::Currency>,

    // TODO -- not sure what this is
    #[serde(
        rename = "paid_price",
        deserialize_with = "de::deserialize_money"
    )]
    pub paid_price: Money<'static, iso::Currency>,

    // The price-per-unit price
    #[serde(
        default,
        rename = "avg_p_p",
        deserialize_with = "de::deserialize_money_optional"
    )]
    pub ppu_price: Option<Money<'static, iso::Currency>>,

    // The price-per-unit quantity
    #[serde(
        default,
        rename = "avg_p_q",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pub ppu_quantity: Option<f64>,

    // The price-per-unit unit
    #[serde(default, rename = "avg_p_u")]
    pub ppu_unit: Option<String>,

    // The quantity of the product added to the user's order
    #[serde(
        default,
        rename = "quantity_in_basket",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub quantity: usize,

    // What the quantity of item is measured in, e.g. "bag", "400g"
    #[serde(rename = "units")]
    pub units: String,
}

#[cfg(test)]
mod tests {

    use super::OrderItem;

    #[test]
    fn test_deserialize() {
        let s = r#"
        {
            "p_id": "15304",
            "product_id": "15304",
            "order_details_id": "321886952",
            "description": null,
            "on_sale": null,
            "default_price": "0.00",
            "defined_price": "5.00",
            "price": "0.00",
            "paid_price": "0.00",
            "avg_p_p": null,
            "avg_p_p_r": "1.25",
            "avg_p_q": "100",
            "show_ind": "1",
            "avg_p_u": "g",
            "quantity_in_basket": "1",
            "p_name": "Analisa Lebanese Cucumbers (strong taste)",
            "s_name": "Lufa Farms Ville Saint-Laurent",
            "image_url": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
            "cat_na": "Vegetables",
            "b_con": "0",
            "weight": "400",
            "c_wei": "0",
            "units": "sac",
            "unit": "g",
            "cat_weight": "0",
            "image_urls": {
                "resized_240x160": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_240x160/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg",
                "resized_690x430": "https://storage.bhs.cloud.ovh.net/v1/AUTH_67da374e12f7497491105aaeeebf4835/public/products/resized_690x430/10878_10878-Concombres-libanais-en-floraison-Les-Fermes-Lufa-Laval-.jpg"
            }
        }
        "#;

        let item: Result<OrderItem, _> = serde_json::from_str(s);
        assert!(item.is_ok());
    }
}
