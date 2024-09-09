// Copyright 2024, Horizen Labs, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Make the set of bag thresholds to be used with pallet-bags-list.

use clap::Parser;
use generate_bags::generate_thresholds;
use std::path::PathBuf;
use subxt::{OnlineClient, SubstrateConfig};

#[derive(Debug, Parser)]
// #[clap(author, version, about)]
struct Opt {
    /// How many bags to generate.
    #[arg(long, default_value_t = 200)]
    n_bags: usize,

    /// Where to write the output.
    output: PathBuf,

    /// The WebSocket URL of the zk-verify node.
    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    node_url: String,
}

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/zk_verify_runtime_metadata.scale")]
pub mod zk_verify {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opt {
        n_bags,
        output,
        node_url,
    } = Opt::parse();

    let api = OnlineClient::<SubstrateConfig>::from_url(node_url).await?;
    let total_issuance = api
        .storage()
        .at_latest()
        .await?
        .fetch(&zk_verify::storage().balances().total_issuance())
        .await?
        .unwrap_or_default();

    generate_thresholds::<zkv_runtime::Runtime>(
        n_bags,
        &output,
        total_issuance,
        zkv_runtime::EXISTENTIAL_DEPOSIT,
    )?;

    Ok(())
}
