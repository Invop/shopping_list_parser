use anyhow::anyhow;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;
pub struct ShoppingItem {
    pub index: usize,
    pub name: String,
    pub quantity: u32,
    pub unit: String,
}
pub fn parse_shopping_list(input: &str) -> anyhow::Result<Vec<ShoppingItem>> {
    if input.trim().is_empty() {
        return Err(anyhow!("input is empty"));
    }

    let mut items = Vec::new();
    let pairs = Grammar::parse(Rule::shopping_list, input)?;
    for pair in pairs {
        if pair.as_rule() == Rule::shopping_list {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::item {
                    items.push(parse_item(inner_pair)?);
                }
            }
        }
    }
    Ok(items)
}

fn parse_item(inner_pair: pest::iterators::Pair<Rule>) -> anyhow::Result<ShoppingItem> {
    let mut item = ShoppingItem {
        index: 0,
        name: String::new(),
        quantity: 0,
        unit: String::new(),
    };
    for element in inner_pair.into_inner() {
        match element.as_rule() {
            Rule::index => item.index = element.as_str().parse()?,
            Rule::name => item.name = element.as_str().to_string(),
            Rule::quantity => item.quantity = element.as_str().trim().parse()?,
            Rule::unit => item.unit = element.as_str().to_string(),
            _ => unreachable!(),
        }
    }
    Ok(item)
}
