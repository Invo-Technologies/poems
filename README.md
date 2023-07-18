# Poems: Proof of the Effective Majority Staked

![Poems Overview](https://github.com/Invo-Technologies/poems/assets/43707795/08d7559e-9161-47f5-a263-307faed2f3cc)

Poems, short for Proof of the Effective Majority Staked, is an innovative approach that introduces a robust and secure mechanism to validate a shared secret within the INVO system. The process involves securely generating and storing account information, including public and private keys. It then transfers a significant portion of this information to another party to achieve a mutually agreed-upon proof, incentivizing consensus.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Instructions](#installation-instructions)
- [Running the Project](#running-the-project)
- [Detailed Walkthrough](#detailed-walkthrough)
- [The Layman's Explanation](#the-laymans-explanation)

## Prerequisites

Before proceeding, ensure you have the following tools installed:

- [Node.js and npm](https://nodejs.org/en/download/)
- [Vite](https://vitejs.dev/guide/#scaffolding-your-first-vite-project)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation Instructions

**Node.js and npm:**

```bash
# Check if you have Node.js and npm installed
node -v
npm -v

# If not, download and install Node.js and npm from the official website
# https://nodejs.org/en/download/
```

**Vite:**

```bash
# Install Vite globally
npm install -g create-vite
```

**Rust:**

```bash
# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Cargo:**

```bash
# Cargo comes with Rust, verify installation
cargo --version
```

## Running the Project

### Running the Rust Program

1. Navigate to the root directory of the Rust project.

```bash
cd poems
```

2. Build the project without running `cargo update`. The `cargo update` command updates your Rust dependencies, but in this project, it can disrupt the code. Therefore, avoid running it.

```bash
cargo build --release
```

3. Execute the project.

```bash
cargo run --all-features
```

### Running the Frontend

Navigate to the directory containing the frontend code.

```bash
cd frontend
```

Install the dependencies.

```bash
npm install
```

Start the Vite server.

```bash
npm run dev
```

Open the displayed local address in your web browser to interact with the project.

## Detailed Walkthrough

Poems functions through a series of transitions that include the generation and storage of account information, mnemonic creation and hashing, the transfer of information to an external party, the exchange of a shared secret, and the final consensus and proof approval. [Explore our step-by-step guide for an in-depth understanding.](https://github.com/Invo-Technologies/poems/blob/main/README.md#transition-1-generation-and-storage-of-account-information)

## The Layman's Explanation

While Poems encompasses a range of sophisticated processes and concepts, we've created an explanation using simple analogies. We use the concept of a game with magic boxes, keys, secret word lists, and magic codes to illustrate the essence of Poems. This light-hearted explanation can help provide a rudimentary understanding of our complex system. [Dive into this fun explanation here!](https://github.com/Invo-Technologies/poems/blob/main/README.md#proof-of-the-effective-majority-stake-protocol-but-explained-in-simpler-terms)