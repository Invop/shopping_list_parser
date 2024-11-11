# Shopping List Parser
### https://crates.io/crates/shopping_list_parser 
### https://docs.rs/shopping_list_parser/latest/shopping_list_parser/
## Description

This project is a **shopping_list_parser** designed for educational purposes. It allows you to parse a structured list of shopping items using a grammar defined in [pest](https://pest.rs/).

## Example Usage

An example of a shopping list that can be parsed by this grammar:

```
1. Apples 5 pcs
2. Bananas 3 kg
3. Mango 1 pcs
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
