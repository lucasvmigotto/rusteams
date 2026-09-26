// rusteams binary entry point.
// Copyright (C) 2026 rusteams contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use rusteams::cli::run;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("rusteams: {err}");
        std::process::exit(1);
    }
}
