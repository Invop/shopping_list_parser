use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

//TODO: impl list item struct
pub fn parse_shopping_list(input: &str) -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::shopping_list, input)?;
    println!("{:?}",got);
    Ok(())
}