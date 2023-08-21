# Project Setup and Execution Instructions

Welcome to our project branch focused on cryptographic operations for the "poems" application. This project is written in Rust and uses Cargo as its build system and package manager. Here, we will guide you through the steps to properly setup, build, and execute this branch.

## Prerequisites

Before starting, you need to have Rust and Cargo installed on your machine. If you don't have them already installed, you can do so by following the instructions on the [Rust installation guide](https://www.rust-lang.org/tools/install).

## Initial Steps

After cloning the repository and navigating to the project root, it is important to clean the project's build artifacts. To do so, run the following command in your terminal:

```bash
cargo clean
```

After cleaning the project, we are ready to build it. You can do this by running:

```bash
cargo build
```

We urge you not to run `cargo update` at this point because the project's dependencies have been locked to specific versions to maintain consistency during development. Running `cargo update` could potentially disrupt the code due to dependency changes.

## Running the Application

Once you have the project built, you can run the branch using the following command:

```bash
cargo run --all-features
```

This command will execute the project with all features enabled.

## Understanding the Project Structure

A critical part of this project is the `src/Generation` directory. It is responsible for testing the backend of our 'poems' program and consists of four key modules:

1. `aes.rs`: This file handles AES (Advanced Encryption Standard) related operations, which are commonly used for symmetric encryption.

2. `bip39.rs`: This module contains functionality for generating mnemonic sentences in accordance with the BIP39 (Bitcoin Improvement Proposals) standard.

3. `rsa.rs`: In this file, we handle RSA (Rivest–Shamir–Adleman) related operations. RSA is a widely used public key cryptographic algorithm.

4. `sha256.rs`: This module is responsible for creating SHA256 (Secure Hash Algorithm 2) hashes, which are used in various security applications and protocols.

These modules are integral to the project as they handle the core cryptographic functionalities.

We hope this README helps you understand how to setup, build, and execute this branch. Should you encounter any difficulties or require further clarification, please feel free to reach out to the development team.