// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]

use rs_tree::rs_tree_run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rs_tree_run().await
}
