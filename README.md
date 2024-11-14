# Shopping List Parser
### https://crates.io/crates/shopping_list_parser 
### https://docs.rs/shopping_list_parser/latest/shopping_list_parser/
## Description

This project is a **shopping_list_parser** designed for educational purposes. It allows you to parse a structured list of shopping items using a grammar defined in [pest](https://pest.rs/).
## Grammar rules
Shopping List Grammar
The following grammar defines the structure of each item in the shopping list, with rules for attributes like item names, quantities, units, and optional descriptions. This structure is used to parse and validate items, ensuring consistent formatting.

Grammar Rules<br>
index: Numeric identifier for each item<br>.
<br>
index = { ASCII_DIGIT+ }<br>
Example: 1, 25<br>
quantity: The amount or count of the item.<br>
<br>
quantity = { ASCII_DIGIT+ }<br>
Example: 2, 5<br>
name: Name of the item, allowing letters, spaces, and hyphens.<br>
<br>
name = { (ASCII_ALPHA | " " | "-")+ }<br>
Example: Apples, Brown Rice<br>
brand: Optional brand information, placed in parentheses.<br>
<br>
brand = { "(" ~ (ASCII_ALPHA | " ")+ ~ ")" }<br>
Example: (Green Organic), (Local Brand)<br>
description: Optional additional details, placed in curly braces.<br>
<br>
description = { "{" ~ (ASCII_ALPHA | ASCII_DIGIT | " ")+ ~ "}" }<br>
Example: {Sweet and crunchy}, {High in fiber}<br>
unit: Measurement unit for the quantity.<br>
<br>
unit = { "kg" | "g" | "ltr" | "ml" | "pcs" | "oz" }<br>
Example: kg, pcs, oz<br>
category: A label for organizing items, placed in square brackets.<br>
<br>
category = { "[" ~ ASCII_ALPHA+ ~ ASCII_DIGIT* ~ "]" }<br>
Example: [Fruits], [Snacks1]<br>
item: Full definition of an item entry, including mandatory and optional elements.<br>
<br>
item = { index ~ "." ~ WHITE_SPACE? ~ name ~ WHITE_SPACE? ~ quantity ~ WHITE_SPACE? ~ unit ~ (WHITE_SPACE? ~ brand)? ~ (WHITE_SPACE? ~ description)? }<br>
Example: 1. Apples 2 kg (Green Organic) {Sweet and crunchy}<br>
shopping_list: A collection of items and categories, with support for whitespace and line breaks.<br>

shopping_list = { SOI ~ ((WHITE_SPACE* ~ (category | item) ~ WHITE_SPACE* ~ NEWLINE?)* ~ EOI) }<br>
```
   1. Apples 2 kg (Green Organic) {Sweet and crunchy}
   2. Milk 1 ltr (Dairy Best)
   3. Bread 1 pcs {Whole grain, freshly baked}
   [Fruits]
   4. Oranges 3 kg
   5. Bananas 1 kg
```
## Features

- Parses a structured shopping list with items, quantities, and units
- Uses the `pest` parser generator library to define the grammar
- Provides a simple API to parse the shopping list

## Getting Started

### Prerequisites

- Rust programming language
- Cargo package manager

### Usage

1. Import the `shopping_list_parser` crate in your Rust project:

   ```rust
   use shopping_list_parser::parse_shopping_list;
   ```

2. Call the `parse_shopping_list` function with a string containing the shopping list:

   ```rust
   let input = "1. Apples 5 pcs\n2. Bananas 3 kg\n3. Mango 1 pcs";
   parse_shopping_list(&input)?;
   ```
