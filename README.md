# Proof of the Effective Majority Staked  

The 'proof of the effective majority' (POEMS) is an innovative approach that enables a staked authority to validate a shared secret through a process that necessitates the exchange of specific records, where neither party possesses any prior knowledge to complete a single proof. 

The proof involves securely generating and storing account information in the INVO system, including public and private keys, using a series of steps such as key generation, mnemonic creation, hashing, and storage. 

A significant portion of this information is subsequently transferred to another party, wherein the key master possesses solely a zero-knowledge proof of the keys they generated, without any knowledge of the key's contents. Only when the external party returns the key to unlock the contents, which both parties are aware of, can a mutually agreed-upon proof be approved—a process that inherently incentivizes both parties to reach consensus. 

The Proof of the Effective Majority (POEMS) is a sophisticated mechanism designed to validate a shared secret within the INVO system.  

## Summary of POEMS 

### Transition 1: Generation and Storage of Account Information 

To initiate the proofing mechanism, the INVO system generates and securely stores the account information. This includes the creation of public and private keys using a reliable key generation algorithm. The generated keys are essential for subsequent steps in the proofing process. 

  

### Transition 2: Mnemonic Creation and Hashing 

In parallel to key generation, a BIP39 mnemonic is created to facilitate account recovery. The mnemonic serves as a backup for the account's private keys. Additionally, a hashing function is applied to the mnemonic, generating a secure 256-bit hash. This hash serves as a vital component in the proofing process. 

  

### Transition 3: Transfer of Information to External Party 

A significant portion of the account information is then transferred to an external party. During this transfer, the key master, who possesses the generated keys, provides a zero-knowledge proof solely based on the keys' creation. Notably, the key master remains unaware of the contents of the keys. 

  

### Transition 4: Exchange of the Shared Secret 

The external party, upon receiving the account information, participates in the exchange of the shared secret. To unlock the contents of the shared secret, the external party returns the key to the key master. Both parties possess prior knowledge of the contents, ensuring mutual agreement and verification. 

  

### Transition 5: Consensus and Proof Approval 


The POEMS process inherently incentivizes both the key master and the external party to reach consensus on the proof. By agreeing on the shared secret and its unlocking process, the proof is verified and approved, indicating a successful completion of the proofing mechanism. 

  
The Proof of the Effective Majority (POEMS) is a robust and secure mechanism that enables the validation of a shared secret within the INVO system. Through a series of steps involving key generation, mnemonic creation, and the exchange of information, the POEMS process ensures consensus and verification between parties involved. 
