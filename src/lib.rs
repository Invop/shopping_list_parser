use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

/// Represents an item in the shopping list.
pub struct ShoppingItem {
    pub index: usize,
    pub name: String,
    pub quantity: u32,
    pub unit: String,
    pub brand: Option<String>,
    pub description: Option<String>,
}

pub struct ShoppingCategory {
    pub name: String,
    pub items: Vec<ShoppingItem>,
}


/// Parses a shopping list from a given string input.
///
/// # Arguments
///
/// * `input` - The input string containing the shopping list to be parsed.
///
/// # Returns
///
/// * `Ok(Vec<ShoppingCategory>)` - A vector of `ShoppingCategory` if parsing is successful.
/// * `Err(anyhow::Error)` - An error if the input is empty or parsing fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The input is empty.
/// - The input cannot be parsed according to the shopping list grammar.
pub fn parse_shopping_list(input: &str) -> anyhow::Result<Vec<ShoppingCategory>> {
    if input.trim().is_empty() {
        return Err(anyhow!("input is empty"));
    }

    let mut categories = Vec::new();
    let mut current_category = None;
    let mut items = Vec::new();

    let pairs = Grammar::parse(Rule::shopping_list, input)?;
    for pair in pairs {
        if pair.as_rule() == Rule::shopping_list {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::category => {
                        if let Some(category) = current_category.take() {
                            categories.push(category);
                        }
                        current_category = Some(ShoppingCategory {
                            name: inner_pair.as_str().to_string(),
                            items: Vec::new(),
                        });
                    }
                    Rule::item => {
                        let item = parse_item(inner_pair)?;
                        if let Some(ref mut category) = current_category {
                            category.items.push(item);
                        } else {
                            items.push(item);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some(category) = current_category {
        categories.push(category);
    }

    Ok(categories)
}
/// Parses an individual item from a given `pest::iterators::Pair`.
///
/// # Arguments
///
/// * `inner_pair` - A `pest::iterators::Pair` representing an item in the shopping list.
///
/// # Returns
///
/// * `Ok(ShoppingItem)` - A `ShoppingItem` if parsing is successful.
/// * `Err(anyhow::Error)` - An error if parsing fails.
///
fn parse_item(inner_pair: pest::iterators::Pair<Rule>) -> anyhow::Result<ShoppingItem> {
    let mut item = ShoppingItem {
        index: 0,
        name: String::new(),
        quantity: 0,
        unit: String::new(),
        brand: None,
        description: None,
    };
    for element in inner_pair.into_inner() {
        match element.as_rule() {
            Rule::index => item.index = element.as_str().parse()?,
            Rule::name => item.name = element.as_str().to_string(),
            Rule::quantity => item.quantity = element.as_str().trim().parse()?,
            Rule::unit => item.unit = element.as_str().to_string(),
            Rule::brand => item.brand = Some(element.as_str().to_string()),
            Rule::description => item.description = Some(element.as_str().to_string()),
            _ => unreachable!(),
        }
    }
    Ok(item)
}
