# POEMS
## Proof of the Effective Majority Staked

The 'proof of the effective majority' (POEMS) is an innovative approach that enables a staked authority to validate a shared secret through a process that necessitates the exchange of specific records, where neither party possesses any prior knowledge to complete a single proof.

Starting with key generation, a unique set of codes is produced, followed by the creation of a mnemonic, a memory aid, to assist in remembering the key. This key then undergoes a process called hashing, converting the key into a fixed size string of characters, ensuring secure data storage. The hashed key is stored securely and is later compared to a signature verification circuit to authenticate its origin. The system's flexibility allows for additional layers of cryptography, capable of tasks like signature verification or generating new keys for private hashes. These hashes can be used in stored procedures in databases, compatible with various cryptographic algorithms like SHA256, RSA, and AES, offering a robust method for secure data handling.

A significant portion of this information is subsequently transferred to another party, wherein the key master possesses solely a zero-knowledge proof of the keys they generated, without any knowledge of the key's contents. Only when the external party returns the key to unlock the contents, which both parties are aware of, can a mutually agreed-upon proof be approved—a process that inherently incentivizes both parties to reach consensus.

The Proof of the Effective Majority (POEMS) is a sophisticated mechanism designed to validate a shared secret within the INVO system.
