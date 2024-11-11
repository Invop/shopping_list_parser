#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use pest::Parser;
    use shopping_list_parser::*;

    #[test]
    fn test_single_item() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::shopping_list, "1.apples5kg\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "1.apples5kg\n");

        let pair = Grammar::parse(Rule::shopping_list, "2. bananas 10 pcs\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "2. bananas 10 pcs\n");
        Ok(())
    }

    #[test]
    fn test_multiple_items() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::shopping_list, "1.apples5kg\n2.bananas10pcs\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "1.apples5kg\n2.bananas10pcs\n");

        let pair = Grammar::parse(Rule::shopping_list, "1. apples 5 kg\n2. bananas 10 pcs\n")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
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

    #[test]
    fn test_parse_shopping_list_single_item() -> anyhow::Result<()> {
        let result = parse_shopping_list("1.apples5kg\n")?;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].name, "apples");
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].unit, "kg");
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_multiple_items() -> anyhow::Result<()> {
        let result = parse_shopping_list("1.apples5kg\n2.bananas10pcs\n")?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].name, "apples");
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].unit, "kg");

        assert_eq!(result[1].index, 2);
        assert_eq!(result[1].name, "bananas");
        assert_eq!(result[1].quantity, 10);
        assert_eq!(result[1].unit, "pcs");
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_with_whitespace() -> anyhow::Result<()> {
        let result = parse_shopping_list("1. apples 5 kg\n2. bananas 10 pcs\n")?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].name, "apples");
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].unit, "kg");

        assert_eq!(result[1].index, 2);
        assert_eq!(result[1].name, "bananas");
        assert_eq!(result[1].quantity, 10);
        assert_eq!(result[1].unit, "pcs");
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_empty() {
        let result = parse_shopping_list("");
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "input is empty");
        }
    }

    #[test]
    fn test_parse_shopping_list_incorrect_format() {
        let result = parse_shopping_list("1apples5kg\n");
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_shopping_list_with_mixed_valid_invalid_items() -> anyhow::Result<()> {
        let result = parse_shopping_list("1.apples5kg\n2.invaliditem\n3.bananas10pcs\n");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_trailing_newline() -> anyhow::Result<()> {
        let result = parse_shopping_list("1.apples5kg\n2.bananas10pcs\n")?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].name, "apples");
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].unit, "kg");

        assert_eq!(result[1].index, 2);
        assert_eq!(result[1].name, "bananas");
        assert_eq!(result[1].quantity, 10);
        assert_eq!(result[1].unit, "pcs");
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_extra_whitespace() -> anyhow::Result<()> {
        let result = parse_shopping_list("1.  apples   5  kg\n 2. bananas   10   pcs\n")?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].index, 1);
        assert_eq!(result[0].name, "apples");
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].unit, "kg");

        assert_eq!(result[1].index, 2);
        assert_eq!(result[1].name, "bananas");
        assert_eq!(result[1].quantity, 10);
        assert_eq!(result[1].unit, "pcs");
        Ok(())
    }

    #[test]
    fn test_parse_shopping_list_single_item_no_unit() {
        let result = parse_shopping_list("1.apples5\n");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_item() {
        let result = parse_shopping_list("1.  \n");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_shopping_list_more_than_one_digit_index() -> anyhow::Result<()> {
        let result = parse_shopping_list("10.oranges15kg\n11.mangoes20pcs\n")?;
        assert_eq!(result.len(), 2);

        assert_eq!(result[0].index, 10);
        assert_eq!(result[0].name, "oranges");
        assert_eq!(result[0].quantity, 15);
        assert_eq!(result[0].unit, "kg");

        assert_eq!(result[1].index, 11);
        assert_eq!(result[1].name, "mangoes");
        assert_eq!(result[1].quantity, 20);
        assert_eq!(result[1].unit, "pcs");
        Ok(())
    }
}
