use powerpack::{Icon, Item};
use std::env;
use std::error::Error;
use std::iter;

fn token_quantity(query: &str) -> Option<String> {
    let args: Result<Vec<f64>, _> = query
        .split_whitespace()
        .map(|arg| arg.parse::<f64>())
        .collect();

    let args = match args {
        Ok(args) => args,
        Err(_) => return None, // Return early if parsing fails
    };

    if args.len() != 3 {
        return None;
    }

    let entry_price = args[0];
    let stop_loss_price = args[1];
    let risk_dollar_amt = args[2];

    let difference = entry_price - stop_loss_price;
    let token_quantity = risk_dollar_amt / difference;

    if token_quantity.is_nan() || token_quantity.is_infinite() {
        None
    } else {
        Some(token_quantity.to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let arg = env::args().nth(1);
    let query = arg.as_deref().unwrap_or("");

    // Create an item to show in the Alfred drop down.
    let item = Item::new("Calculate")
        .subtitle(format!("Your query was '{}'", query))
        .arg(token_quantity(query).unwrap_or_default())
        .icon(Icon::with_image("icon.png"));

    // Output the item to Alfred!
    powerpack::output(iter::once(item))?;

    Ok(())
}
