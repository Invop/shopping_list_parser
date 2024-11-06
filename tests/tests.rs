#[cfg(test)]
mod tests {
    use pest::Parser;
    use anyhow::anyhow;
    use shopping_list_parser::*;


    #[test]
    fn test_single_item() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::shopping_list, "1.apples5kg\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "1.apples5kg\n");

        let pair = Grammar::parse(Rule::shopping_list, "2. bananas 10 pcs\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "2. bananas 10 pcs\n");
        Ok(())
    }

    #[test]
    fn test_multiple_items() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::shopping_list, "1.apples5kg\n2.bananas10pcs\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "1.apples5kg\n2.bananas10pcs\n");

        let pair = Grammar::parse(Rule::shopping_list, "1. apples 5 kg\n2. bananas 10 pcs\n")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "1. apples 5 kg\n2. bananas 10 pcs\n");
        Ok(())
    }

    #[test]
    fn test_incorrect_format() {
        // Missing dot between index and item name
        let pair = Grammar::parse(Rule::shopping_list, "1apples5kg\n");
        assert!(pair.is_err());

        // Invalid unit
        let pair = Grammar::parse(Rule::shopping_list, "1.apples5cups\n");
        assert!(pair.is_err());

        // Missing quantity
        let pair = Grammar::parse(Rule::shopping_list, "1.appleskg\n");
        assert!(pair.is_err());
    }
}