use powerpack::{Icon, Item};
use std::env;
use std::error::Error;
use std::iter;

fn token_quantity(args: &[f64]) -> Option<String> {
    if args.len() != 3 {
        return None;
    }

    let (entry_price, stop_loss_price, risk_dollar_amt) = (args[0], args[1], args[2]);

    let difference = entry_price - stop_loss_price;
    let token_quantity = risk_dollar_amt / difference;

    if token_quantity.is_finite() {
        Some(token_quantity.to_string())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1).unwrap_or_default();
    let args: Vec<f64> = query
        .split_whitespace()
        .map(|arg| arg.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()
        .unwrap_or_default();

    // Create an item to show in the Alfred drop down.
    let item = Item::new("entry_price stoploss_price dollar_risk")
        .subtitle(format!("calculate '{}'", query))
        .arg(token_quantity(&args).unwrap_or_default())
        .icon(Icon::with_image("icon.png"));

    // Output the item to Alfred!
    powerpack::output(iter::once(item))?;

    Ok(())
}
