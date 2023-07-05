use powerpack::{Icon, Item};
use std::env;
use std::error::Error;
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let arg = env::args().nth(1);
    let query = arg.as_deref().unwrap_or("");

    // Create an item to show in the Alfred drop down.
    let item = Item::new("Hello World!")
        .subtitle(format!("Your query was '{}'", query))
        .arg(query.to_string())
        .icon(Icon::with_image("icon.png"));

    // Output the item to Alfred!
    powerpack::output(iter::once(item))?;

    Ok(())
}
