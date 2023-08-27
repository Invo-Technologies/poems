
# Poems: Proof of the Effective Majority Staked

![Poems Overview](https://github.com/Invo-Technologies/poems/assets/43707795/08d7559e-9161-47f5-a263-307faed2f3cc)

Poems, short for Proof of the Effective Majority Staked, is an innovative approach that introduces a robust and secure mechanism to validate a shared secret within the INVO system. The process involves securely generating and storing account information, including public and private keys. It then transfers a significant portion of this information to another party to achieve a mutually agreed-upon proof, incentivizing consensus.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Instructions](#installation-instructions)
- [Running the Project](#running-the-project)
- [Usage](#usage)
- [Understanding POEMS](#understanding-poems)
- [The Easy Explanation](#the-easy-explanation)

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

**Rust and Cargo:**

```bash
# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cargo comes with Rust, verify installation
cargo --version
```

## Running the Project

### Installing the Poems CLI

1. Navigate to the root directory of the Poems project.

```bash
cd poems
```

2. Build and install the project.

```bash
cargo install --path .
```

This will install the `poems` command-line tool on your system.

### Usage

After installing, you can use the `poems` command followed by a subcommand:

- To execute the decryption program:

```bash
poems decrypt
```

- To execute the registration/key generation without the decryption program:

```bash
poems registration
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

## Understanding POEMS

... [rest of the content remains unchanged]
```

This updated `README.md` provides clear instructions on how to install and use the `poems` command-line tool. After making these changes to your README, anyone who clones or downloads your project will have a clear guide on how to set up and use the tool.

## Understanding POEMS

### Proof of the Effective Majority Staked

The 'proof of the effective majority' (POEMS) is an innovative approach that enables a staked authority to validate a shared secret through a process that necessitates the exchange of specific records, where neither party possesses any prior knowledge to complete a single proof. The proof involves securely generating and storing account information in the INVO system, including public and private keys, using a series of steps such as key generation, mnemonic creation, hashing, and storage. A significant portion of this information is subsequently transferred to another party, wherein the key master possesses solely a zero-knowledge proof of the keys they generated, without any knowledge of the key's contents. Only when the external party returns the key to unlock the contents, which both parties are aware of, can a mutually agreed-upon proof be approved—a process that inherently incentivizes both parties to reach consensus.

The Proof of the Effective Majority (POEMS) is a sophisticated mechanism designed to validate a shared secret within the INVO system.

### Summary of POEMS 

![POEMS Summary](https://github.com/Invo-Technologies/poems/assets/43707795/08d7559e-9161-47f5-a263-307faed2f3cc)

#### Transition 1: Generation and Storage of Account Information

To initiate the proofing mechanism, the INVO system generates and securely stores the account information. This includes the creation of public and private keys using a reliable key generation algorithm. The generated keys are essential for subsequent steps in the proofing process.

#### Transition 2: Mnemonic Creation and Hashing

In parallel to key generation, a BIP39 mnemonic is created to facilitate account recovery. The mnemonic serves as a backup for the account's private keys. Additionally, a hashing function is applied to the mnemonic, generating a secure 256-bit hash. This hash serves as a vital component in the proofing process.

#### Transition 3: Transfer of Information to External Party

A significant portion of the account information is then transferred to an external party. During this transfer, the key master, who possesses the generated keys, provides a zero-knowledge proof solely based on the keys' creation. Notably, the key master remains unaware of the contents of the keys.

#### Transition 4: Exchange of the Shared Secret

The external party, upon receiving the account information, participates in the exchange of the shared secret. To unlock the contents of the shared secret, the external party returns the key to the key master. Both parties possess prior knowledge of the contents, ensuring mutual agreement and verification.

#### Transition 5: Consensus and Proof Approval

The POEMS process inherently incentivizes both the key master and the external party to reach consensus on the proof. By agreeing on the shared secret and its unlocking process, the proof is verified and approved, indicating a successful completion of the proofing mechanism.

The Proof of the Effective Majority (POEMS) is a robust and secure mechanism that enables the validation of a shared secret within the INVO system. Through a series of steps involving key generation, mnemonic creation, and the exchange of information, the POEMS process ensures consensus and verification between parties involved.

## The Easy Explanation

Imagine we're playing a game 🎮 ! In this game, I make a magic box 🎁 and magic keys 🔑🔑. I also make an invisible, secret word list 📜. This word list is super special because only you and I know about it , and it can help us make more keys if we ever lose them. 🗑️
 

Now, I use the secret word list 📜 and make a magic code 🍭. **This code is connected to the word list but doesn't give it away**. I give you the magic box 🎁, the keys 🔑🔑, and the magic code 🍭. **But I keep the word list to myself** 📜. 


The magic box 🎁 can only be opened with the keys 🔑🔑, **but the keys are useless without the secret word list** 📜.
 

This is where it gets interesting!🎉 Even though you have the box 🎁 and keys 🔑🔑, you can't open the box without the secret word list 📜. So, you take the magic code 🍭 that I gave you, and you use it to make your own word list 📃. And guess what? Your word list matches mine!  📜 == 📃 


Now that you've made your own word list 📃, you can use it to make a new key 🗝️. And this key opens the magic box 🎁! This proves that you made the same key that I had secretly stored 🔑🔑 = 🗝️🗝️. When the box opens, I can use the secret code 🍭 you used to check that everything matches up! 
 

This way, we've proven that you're the only one who could have opened the box 🎁, because only you and I knew the secret code 🍭, and you were the one who made the word list 📃 that could make the magic key 🗝️. That's what we call 'Proof of the Effective Majority Staked' or POEMS.
