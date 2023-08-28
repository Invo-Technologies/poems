# `poems/src` Directory Instructions

Welcome to the `src` directory of the `poems` application! This directory is where all the source code of the application lives. The directory structure is organized and layered to ensure maintainability, scalability, and ease of use. This README provides an overview of the `src` directory, its contents, and how to effectively utilize the provided codebase.

## Prerequisites

This guide assumes that you have already setup the `poems` project as per the instructions in the main README.md file located in the project root. If you haven't done this yet, please navigate back to the root directory and follow the instructions there before proceeding.

## Directory Structure

Within the `src` directory, we have two main subdirectories: `Generation` and `Stored Procedures`, along with `main.rs` and `lib.rs` files. Here's an overview of each:

1. `Generation`: This directory is home to modules related to cryptographic operations. It includes functionality for generating keys, creating hashes, and other encryption related tasks.

2. `Stored Procedures`: Here, you'll find modules responsible for handling database operations and interactions.

The `main.rs` and `lib.rs` files serve as the central point for orchestrating the entire application:

- `main.rs`: This is the entry point of the application. It primarily coordinates and initializes necessary services and functionalities.

- `lib.rs`: This file exports the modules and functions necessary for other parts of the application to function properly.

## Running the Application

To run the application from this directory, please use the following command:

```bash
cargo run --all-features
```

Ensure you are in the root directory of the project when running this command.

## Understanding the Codebase

The `Generation` and `Stored Procedures` directories play significant roles in the application. While `Generation` handles cryptographic operations, `Stored Procedures` manages the database operations. This separation of responsibilities helps to maintain a clean and understandable codebase.

Remember, each module in the `Generation` directory focuses on a specific aspect of cryptography:

- `aes.rs`: AES (Advanced Encryption Standard) related operations.
- `bip39.rs`: Functionality for generating BIP39 (Bitcoin Improvement Proposals) standard mnemonic sentences.
- `rsa.rs`: RSA (Rivest–Shamir–Adleman) related operations.
- `sha256.rs`: Creation of SHA256 (Secure Hash Algorithm 2) hashes.

Together, they provide the backbone for the secure operation of the `poems` application.

This README should help you navigate and understand the `src` directory. For any queries or concerns, don't hesitate to contact the development team.