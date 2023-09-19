<h1 align="center">
    <picture>
        <source media="(prefers-color-scheme: light)" srcset="https://d2yxqfr8upg55w.cloudfront.net/assets/img/DL_logo_1800 X 400.png">
        <source media="(prefers-color-scheme: dark)" srcset="https://d2yxqfr8upg55w.cloudfront.net/assets/img/DL_LOGO_dark_theme.png">
        <img width='60%' height='60%' src="https://d2yxqfr8upg55w.cloudfront.net/assets/img/DL_logo_1800 x 400.png" alt="DL logo">
    </picture>
</h1>

# Substream

Substreams is a powerful blockchain indexing technology, developed for **The Graph** Network.

<h4 align="center">

  <a href="https://discord.com/invite/dapplooker">
    <img src="https://d2yxqfr8upg55w.cloudfront.net/assets/img/substream-logo.jpeg">
  </a>
  </a>
</h4>

# Substream features:-

- Substreams enables developers to write Rust modules
- It provides extremely high-performance indexing by virtue of parallelization, in a streaming-first fashion.
- Low-cost caching and archiving of blockchain data, high throughput processing, and cursor-based reorgs handling.
- The ability to store and process blockchain data using advanced parallelization techniques, making the processed data available for various types of data stores or real-time systems.

## Pre-requisites

### Getting started with Rust

- [Half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

### Getting started with Substreams

- [YouTube: Introducing Substreams](https://www.youtube.com/watch?v=qWxffTKpciU)
- [Developer Docs](https://substreams.streamingfast.io/)

## Local Setup Guide for Substreams

Follow these steps to set up Substreams locally using a different approach:

1. Begin by creating a new folder and cloning the repository. You can clone it from [this link](https://github.com/streamingfast/substreams-template/generate).

2. Install the necessary dependencies:

   - Install the Rust programming language, which is used for developing custom logic. You can install Rust in various ways, but for simplicity, execute the following commands:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```

   - Install `protoc`, the Protocol Buffer compiler required for generating code in Rust and other languages from the protobuf definitions. Refer to the official [protocol buffer compiler documentation](https://grpc.io/docs/protoc-installation/) for installation instructions.

   - Install `protoc-gen-prost`, a tool that helps generate Rust structures from protobuf definitions for use in Substreams modules. Install it by running:

     ```bash
     cargo install protoc-gen-prost
     ```

     > Note: If you forget to install `protoc` before generating the definitions, you may encounter an error mentioning `cmake` not being defined. Installing `protoc` is necessary as a fallback.

   - Install `buf`, a tool that simplifies the generation of typed structures in any language. It simplifies the usage of `protoc` and supports Substreams packages. Visit [https://buf.build](https://buf.build) for installation instructions.

3. Obtain the `substreams` CLI tool:

   - For macOS users, install it using `brew`:

     ```bash
     brew install streamingfast/tap/substreams
     ```

   - Alternatively, download the pre-compiled binary for your platform:

     ```bash
     # Replace the URL with the correct binary for your platform
     wget https://github.com/streamingfast/substreams/releases/download/v0.0.12/substreams_0.0.12_linux_x86_64.tar.gz
     tar -xzvf substreams_0.0.12_linux_x86_64.tar.gz
     export PATH="`pwd`:$PATH"
     ```

     > Make sure to visit [https://github.com/streamingfast/substreams/releases](https://github.com/streamingfast/substreams/releases) and use the latest available release.

4. Validate the installation by checking if the `substreams` CLI works correctly:

   ```bash
   substreams -v
   version (...)

   ```

5. Generating Protobuf:

Use the following command to generate Protobuf code:

```
substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
```

6. Compilation:

Now, it's time to build the WASM binary and Protobuf definitions. Execute the following command:

```
cargo build --target wasm32-unknown-unknown --release
```

7. Running the Substream:

Finally, you can run the example Substream. Make sure you are in the project's root directory before executing the following commands:

To run map module

```
substreams run -e <substream_endpoint> substreams.yaml map_trx --start-block 17045218 --stop-block +1
```

# Create your own Substream

### Create Substreams manifest

- To create a "Substreams module", you must first create the manifest file. This example manifest includes the minimal required fields to demonstrate the core values that you must provide.
- To use the example manifest, copy and paste it into a new file named substreams.yaml

```
specVersion: v0.1.0
package:
  name: "network_substream"
  version: v1.0.0

protobuf:
  files:
    - block_meta.proto
  importPaths:
    - ./proto

binaries:
  default:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/substreams.wasm

modules:
  - name: map_block
    kind: map
    initialBlock: 17045218
    inputs:
      - source: sf.ethereum.type.v2.Block
    output:
      type: proto:acme.BlockHeader

```

### Create Rust manifest file

- To complete your new Substreams module, you must also create a Rust manifest file. <br>
- To use the example Rust manifest file, copy and paste its content into a new file named Cargo.toml. Save this file in the root directory of your Substreams module. <br>
- It's important to provide a unique and useful value for the "name" field and to make sure that crate-type = ["cdylib"] is defined so the WASM is generated.

```
[package]
name = "network_substream"
version = "0.1.0"
edition = "2021"

[lib]
name = "substreams"
crate-type = ["cdylib"]

[dependencies]
substreams = "0.5.0"
substreams-ethereum = "0.9.0"
substreams-entity-change = {git = "https://github.com/streamingfast/substreams-entity-change/", branch = "develop"}
prost = "^0.11.0"


[profile.release]
lto = true
opt-level = 'z'
strip = "debuginfo"

[profile.dev]
rustc_flags = ["--allow-non-snake-case"]
```

### Create protobufs

- Substreams modules are required to output protobuf encoded messages.
- Copy and paste the content for the example protobuf definition into a new file named block_meta.proto and save it to a proto directory in the root directory of your Substreams module.
- Learn more about the details of Google Protocol Buffers in the official documentation provided by [Google](https://protobuf.dev/).

```
syntax = "proto3";

package acme;

message BigInt {
  bytes bytes = 1;
}

// ## Block Details ##
message BlockHeader {
  string id = 1;
  string parentHash = 2;
  string uncleHash = 3;
  uint64 nonce = 4;
  bytes receipt_root = 5;
  uint64 number = 6;
  uint64 gasLimit = 7;
  uint64 gasUsed = 8;
  int64 timestamp = 9;
  uint64 size = 10;
}
```

- The mod.rs file located in the src/pb directory of the Substreams Template example is responsible for exporting the freshly generated Rust code.

```
#[path = "acme.rs"]
#[allow(dead_code)]
pub mod acme;
```

Use the substreams protogen command to generate the Rust code to communicate with the protobuf:

```bash
 substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"
```

### Create Substreams module handlers

- Your Substreams module must contain a Rust library that houses the module handlers, the code that is invoked to perform your customized logic. These handlers are responsible for handling blockchain data injected into the module at runtime,
- To include this example module handler in your module, copy it into a new Rust source code file named lib.rs within the src directory.

```
mod pb;
mod db;
mod tables;

use pb::acme;
use crate::tables::Tables;
use pb::acme::{Transaction, TransactionList, BlockHeader, ContractList, Contract};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::TransactionTraceStatus;
use substreams::store::{StoreNew, StoreSetProto};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams::store::StoreSet;
use hex;
use substreams_ethereum::pb::eth::v2::BigInt;

substreams_ethereum::init!();

fn base_64_to_hex<T: std::convert::AsRef<[u8]>>(num:T) -> String {
    let num = hex::encode(&num);
    let num = num.to_string();
     format!("0x{}", &num)
}

#[substreams::handlers::map]
fn map_block(block: eth::Block) -> Result<BlockHeader, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();
    Ok(BlockHeader {
        id: base_64_to_hex(&block.hash),
        parentHash: base_64_to_hex(&header.parent_hash),
        uncleHash: base_64_to_hex(&header.parent_hash),
        receiptRoot: header.receipt_root.clone(),
        gasLimit: header.gas_limit,
        gasUsed: header.gas_used,
        number: block.number,
        nonce: header.nonce,
        timestamp: header.timestamp.clone().unwrap().seconds,
        size: block.size,

    })
}
```

Compile your Substreams module.

```bash
cargo build --release --target wasm32-unknown-unknown
```

### Execute

```
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_block --start-block 17045218 --stop-block +1
```

# Deploy a Substream based Subgraph to the Hosted Service

### Create a Subgraph on the Hosted Service

- Before deploying the subgraph, you need to create it in The Graph Explorer. Go to the dashboard and click on the 'Add Subgraph' button and fill in the information

### Create Subgraph Manifest file

- create a subgraph.yaml file in the root of your substream
- To use the example manifest, copy and paste it into a new file named subgraph.yaml

```
specVersion: 0.0.5
schema:
  file: ./schema.graphql
dataSources:
  - kind: substreams
    name: example
    network: mainnet
    source:
      package:
        moduleName: graph_out
        file: ./network-substream-v1.0.0.spkg
    mapping:
      apiVersion: 0.0.5
      kind: substreams/graph-entities
```

### Create The GraphQL Schema file

#### Step 1 => Create GraphQL Schema file

- The schema for your subgraph is in the file schema.graphql. GraphQL schemas are defined using the GraphQL interface definition language
- To use the example, copy and paste it into a new file named schema.graphql

```
type Block @entity {
  id: String!
  parentHash: String!
  uncleHash: String!
  nonce: BigInt!
  receiptRoot: Bytes!
  number: BigInt!
  gasLimit: BigInt!
  gasUsed: BigInt!
  timestamp: BigInt!
  size: BigInt!
}
```

#### Step 2 => Create graph_out module

- Include the graph-out module in your lib.rs file.

- The graph_out function calls the to_entity_changes() method on the Tables object because it wants to store the entity changes in a database. The Tables object contains a table for each entity type, and the to_entity_changes() method will add the entity changes to the appropriate table.

- Once the entity changes have been added to the table, they can be queried and analyzed using graph analytics tools.

```
#[substreams::handlers::map]
pub fn graph_out(map_trx: TransactionList, map_block: BlockHeader, map_contract: ContractList) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    db::create_block_entity(&mut tables, &map_block);
    Ok(tables.to_entity_changes())
}
```

- Don't forget to include the graph_out module in your substreams.yaml file.

```
- name: graph_out
    kind: map
    initialBlock: 17045218
    inputs:
      - map: map_block
    output:
      type: proto:sf.substreams.entity.v1.EntityChanges

```

#### Step 3 => Create a dedicated file where you can write the logic for creating tables and entities.

- To utilize the provided example, simply copy and paste it into a new file named db.rs.

```
use crate::acme::{Transaction, TransactionList, BlockHeader, ContractList, Contract};
use crate::tables::Tables;

pub fn create_block_entity(tables: &mut Tables, block:&BlockHeader) {
    tables
        .create_row("Block", &block.id)
        .set("id", format!("0x{}", &block.id))
        .set("parentHash", &block.parentHash)
        .set("uncleHash", &block.uncleHash)
        .set("nonce", block.nonce)
        .set("receiptRoot", &block.receiptRoot)
        .set("number", block.number)
        .set("gasLimit", block.gasLimit)
        .set("gasUsed", block.gasUsed)
        .set("timestamp", block.timestamp)
        .set("size", block.size);
}

```

#### Step 4 => Create Package file

- Build your substream and generate the package file using the following command:

```bash
$ substreams pack ./substreams.yaml
```

#### Step 5 => Initialize the subgraph

- Begin by running the "Initialize subgraph" command to set up the subgraph.

```bash
graph init --product hosted-service dapplooker/example
```

- Choose the Substreams option and provide the path to your package file.
- Execute the graph-build command to create the necessary build files for the subgraph
- Finally, run the DEPLOY SUBGRAPH command to deploy the subgraph to the hosted service.

#### Step 6 => Deploy the subgraph

```bash
graph deploy --product hosted-service dapplooker/example
```

### Sample Substreams

- [Subtreams Template (NFT)](https://github.com/streamingfast/substreams-template)
- [Uniswap v3](https://github.com/streamingfast/substreams-uniswap-v3)
- [Compound v2](https://github.com/0xbe1/compoundv2-substreams)

  **If you find the list helpful, please make sure to ⭐ star it!**
