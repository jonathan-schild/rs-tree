/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use anyhow::Error;
use rs_tree::rs_tree_run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    rs_tree_run().await
}
