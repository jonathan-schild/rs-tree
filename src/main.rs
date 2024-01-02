// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]

use anyhow::Error;
use rs_tree::rs_tree_run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    rs_tree_run().await
}
