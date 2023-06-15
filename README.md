# Substream

Substreams is a powerful blockchain indexing technology, developed for **The Graph** Network.

<h4 align="center">

  <a href="https://discord.com/invite/dapplooker">
    <img src="https://media.discordapp.net/attachments/1045635544580886528/1118846470389518336/LinkedIn_background.png?width=1439&height=359">
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
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Tiago's Rust first steps](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)
- [Rust By Example](https://github.com/rust-lang/rust-by-example)
- [Rust By Practice](https://github.com/sunface/rust-by-practice)
- [Rustlings](https://github.com/rust-lang/rustlings)
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
substreams run -e api-dev.streamingfast.io:443 substreams.yaml map_transfers --start-block 12292922 --stop-block +1
```

To run the store module (and the map module in the background):

```
substreams run -e api-dev.streamingfast.io:443 substreams.yaml store_transfers --start-block 12292922 --stop-block +1
```

### Sample Substreams

- [Subtreams Template (NFT)](https://github.com/streamingfast/substreams-template)
- [Uniswap v3](https://github.com/streamingfast/substreams-uniswap-v3)
- [Compound v2](https://github.com/0xbe1/compoundv2-substreams)

  **If you find the list helpful, please make sure to ⭐ star it!**
