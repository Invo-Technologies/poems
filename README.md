# Poems: Proof of the Effective Majority Staked

Welcome to the 'Proof of the Effective Majority Staked' (POEMS) repository. POEMS presents an innovative approach that enables a staked authority to validate a shared secret through a series of exchanges and validations, where neither party possesses any prior knowledge to complete a single proof.

The entire process is securely facilitated within the INVO system. It involves generating and storing account information, including public and private keys, using key generation, mnemonic creation, hashing, and storage. A substantial portion of this information is later transferred to another party, wherein the key master solely possesses a zero-knowledge proof of the keys they generated, without any knowledge of the key's contents.

Only when the external party returns the key to unlock the contents, which both parties are aware of, can a mutually agreed-upon proof be approved. This process inherently incentivizes both parties to reach consensus. The POEMS mechanism is a sophisticated design intended to validate a shared secret within the INVO system.

![POEMS Overview](https://github.com/Invo-Technologies/poems/assets/43707795/08d7559e-9161-47f5-a263-307faed2f3cc)

## Step-by-Step Overview of POEMS

### Transition 1: Generation and Storage of Account Information

The INVO system initiates the proofing mechanism by generating and securely storing account information. This information includes public and private keys created using a reliable key generation algorithm, and these keys play a crucial role in subsequent steps of the proofing process.

### Transition 2: Mnemonic Creation and Hashing

Simultaneously with key generation, a BIP39 mnemonic is created to facilitate account recovery. This mnemonic serves as a backup for the account's private keys. Moreover, a hashing function is applied to the mnemonic, generating a secure 256-bit hash, a crucial component in the proofing process.

### Transition 3: Transfer of Information to External Party

A significant portion of the account information is transferred to an external party. During this transfer, the key master, holding the generated keys, provides a zero-knowledge proof based solely on the creation of the keys. Importantly, the key master remains unaware of the contents of the keys.

### Transition 4: Exchange of the Shared Secret

Upon receiving the account information, the external party participates in the exchange of the shared secret. To unlock the contents of the shared secret, the external party returns the key to the key master. Both parties have prior knowledge of the contents, ensuring mutual agreement and verification.

### Transition 5: Consensus and Proof Approval

The POEMS process naturally incentivizes both the key master and the external party to reach consensus on the proof. By agreeing on the shared secret and its unlocking process, the proof is verified and approved, signaling the successful completion of the proofing mechanism.

POEMS is a robust and secure mechanism designed to validate a shared secret within the INVO system. Through a series of steps involving key generation, mnemonic creation, and the exchange of information, the POEMS process ensures consensus and verification between the involved parties.

## Proof of the Effective Majority Stake protocol, Simplified

Imagine we're playing a game 🎮! In this game, I make a magic box 🎁 and magic keys 🔑🔑. I also create a special secret word list 📜, known only to you and me. This list can help us recreate the keys if we ever lose them.

I use this secret word list 📜 to create a magic code 🍭, which is related to the word list but doesn't reveal it. I give you the magic box 🎁, the keys 🔑🔑, and the magic code 🍭, while the word list stays with me.

The magic box 🎁 can only be opened with the keys 🔑🔑, but they are useless without the secret word list 📜.

Here's the exciting part! 🎉 Even though you have the box 🎁 and keys 🔑🔑, you can't open the box without the secret word list 📜. So, you take the magic code 🍭 that I gave you, and you use it to create your own word list 📃. Surprise, surprise, your word list matches mine! 📜 == 📃

Now that you've created your word list 📃, you can use it to forge a new key 🗝️, which opens the magic box 🎁! This proves that you have successfully recreated the key I had secretly stored 🔑🔑 = 🗝️🗝️. When the box opens, I use the secret code 🍭 you used to check that everything matches!

This way, we've proven that you're the only one who could have opened the box 🎁, because only you and I knew the secret code 🍭, and you created the word list 📃 that made the magic key 🗝️. That's what we call 'Proof of the Effective Majority Staked' or POEMS.