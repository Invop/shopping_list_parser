#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use pest::Parser;
    use shopping_list_parser::*;

    #[test]
    fn test_index() -> anyhow::Result<()> {
        // Valid cases
        let pair = Grammar::parse(Rule::index, "123")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "123");

        // Test invalid index
        assert!(Grammar::parse(Rule::index, "abc").is_err());  // Only digits allowed
        assert!(Grammar::parse(Rule::index, "").is_err());     // Empty not allowed

        Ok(())
    }

    #[test]
    fn test_quantity() -> anyhow::Result<()> {
        // Valid cases
        let pair = Grammar::parse(Rule::quantity, "500")?.next().ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "500");

        // Test invalid quantity
        assert!(Grammar::parse(Rule::quantity, "abc").is_err());  // Only digits allowed
        assert!(Grammar::parse(Rule::quantity, "").is_err());     // Empty not allowed

        Ok(())
    }

    #[test]
    fn test_name() -> anyhow::Result<()> {
        let valid_names = vec![
            "Milk",
            "Fresh Milk",
            "Ultra-Fresh Milk",
        ];

        for name in valid_names {
            let pair = Grammar::parse(Rule::name, name)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), name);
        }

        // Test invalid names
        assert!(Grammar::parse(Rule::name, "").is_err());         // Empty not allowed
        assert!(Grammar::parse(Rule::name, "123").is_err());      // Only alpha, space, and hyphen allowed

        Ok(())
    }

    #[test]
    fn test_brand() -> anyhow::Result<()> {
        let valid_brands = vec![
            "(Nestle)",
            "(Coca Cola)",
        ];

        for brand in valid_brands {
            let pair = Grammar::parse(Rule::brand, brand)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), brand);
        }

        // Test invalid brands
        assert!(Grammar::parse(Rule::brand, "Nestle").is_err());      // Missing parentheses
        assert!(Grammar::parse(Rule::brand, "(123)").is_err());       // Numbers not allowed
        assert!(Grammar::parse(Rule::brand, "()").is_err());          // Empty not allowed

        Ok(())
    }

    #[test]
    fn test_description() -> anyhow::Result<()> {
        let valid_descriptions = vec![
            "{Fresh and cold}",
            "{2 percent fat}",
        ];

        for desc in valid_descriptions {
            let pair = Grammar::parse(Rule::description, desc)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), desc);
        }

        // Test invalid descriptions
        assert!(Grammar::parse(Rule::description, "no braces").is_err());  // Missing braces
        assert!(Grammar::parse(Rule::description, "{}").is_err());         // Empty not allowed

        Ok(())
    }

    #[test]
    fn test_unit() -> anyhow::Result<()> {
        let valid_units = vec!["kg", "g", "ltr", "ml", "pcs", "oz"];

        for unit in valid_units {
            let pair = Grammar::parse(Rule::unit, unit)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), unit);
        }

        // Test invalid units
        assert!(Grammar::parse(Rule::unit, "pound").is_err());  // Not in allowed units
        assert!(Grammar::parse(Rule::unit, "").is_err());       // Empty not allowed

        Ok(())
    }

    #[test]
    fn test_category() -> anyhow::Result<()> {
        let valid_categories = vec![
            "[DAIRY]",
            "[DAIRY1]",
            "[BEVERAGES2]",
            // Based on the grammar, lowercase is actually allowed
            "[dairy]",
            "[beverages]",
        ];

        for category in valid_categories {
            let pair = Grammar::parse(Rule::category, category)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), category);
        }

        // Test invalid categories
        assert!(Grammar::parse(Rule::category, "DAIRY").is_err());       // Missing brackets
        assert!(Grammar::parse(Rule::category, "[]").is_err());          // Empty not allowed
        assert!(Grammar::parse(Rule::category, "[DAIRY-1]").is_err());   // Hyphen not allowed

        Ok(())
    }

    #[test]
    fn test_item() -> anyhow::Result<()> {
        let valid_items = vec![
            "1. Milk 1 ltr",
            "2. Fresh Milk 2 ltr (Nestle)",
            "3. Ultra-Fresh Milk 3 ltr (Nestle) {Fresh and cold}",
            // No whitespace tests
            "1.Milk1ltr",
            "2.FreshMilk2ltr(Nestle)",
            "3.Ultra-FreshMilk3ltr(Nestle){Freshandcold}",
        ];

        for item in valid_items {
            let pair = Grammar::parse(Rule::item, item)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), item);
        }

        // Test invalid items
        assert!(Grammar::parse(Rule::item, "Milk 1 ltr").is_err());     // Missing index
        assert!(Grammar::parse(Rule::item, "1.").is_err());             // Missing name, quantity, unit
        assert!(Grammar::parse(Rule::item, "1. Milk").is_err());        // Missing quantity and unit

        Ok(())
    }

    #[test]
    fn test_shopping_list() -> anyhow::Result<()> {
        let valid_lists = vec![
            // Single category with items
            "[DAIRY]\n1. Milk 1 ltr\n2. Yogurt 500 g",
            // Multiple categories with items
            "[DAIRY]\n1. Milk 1 ltr\n[BEVERAGES]\n2. Cola 2 ltr",
            // Items without category
            "1. Milk 1 ltr\n2. Cola 2 ltr",
            // Empty list is valid
            "",
            // Single category is valid
            "[DAIRY]",
            // Optional whitespace tests
            "1. Milk 1 ltr \n2. Cola 2 ltr",
            "1. Milk 1 ltr\n  2. Cola 2 ltr",
        ];

        for list in valid_lists {
            let pair = Grammar::parse(Rule::shopping_list, list)?.next().ok_or_else(|| anyhow!("no pair"))?;
            assert_eq!(pair.as_str(), list);
        }

        // Test invalid lists
        assert!(Grammar::parse(Rule::shopping_list, "Invalid List").is_err());  // Invalid format
        assert!(Grammar::parse(Rule::shopping_list, "Milk 1 ltr").is_err());   // Missing index
        assert!(Grammar::parse(Rule::shopping_list, "[INVALID@]").is_err());   // Invalid category format

        Ok(())
    }


}