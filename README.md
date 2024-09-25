# Turbin3 Program

<h1 align="center">
  <br>
  <a href="https://turbin3.com"><img src="./img/image.png" alt="turbin3_logo" width="200"></a>
  <br>
  Cohort 2024 Q3
  <br>
</h1>

# Content

- [Rust tool](./cli_tool/Turbin3_Prereq_rust/)
- [Typescript tool](./cli_tool/Turbin3_Prereq_typescript/)

This project is both typescript and Rust-based CLI tool and test suite designed to interact with a Solana program using an Anchor IDL. It provides functionalities such as:

- Generating Solana keypairs
- Converting between base58 encoded private keys and wallet byte arrays
- Requesting SOL airdrops
- Transferring SOL
- Enrolling in a Solana program

- SPL Token
  - [token initailisation](./spl_token_and%20_nft/cluster1/spl_init.ts)
  - [token Metadata](./spl_token_and%20_nft/cluster1/spl_metadata.ts)
  - [token Miniting](./spl_token_and%20_nft/cluster1/spl_mint.ts)
  - [token Transfer](./spl_token_and%20_nft/cluster1/spl_mint.ts)
- NFT

  - [token image creation and uploading](./spl_token_and%20_nft/cluster1/nft_image.ts)
  - [token Metadata](./spl_token_and%20_nft/cluster1/nft_metadata.ts)
  - [token Miniting](./spl_token_and%20_nft/cluster1/nft_mint.ts)

- Vault Program

  - [Vault Program](./vault-program/programs/vault-program/src/lib.rs)
  - [vault Test](./vault-program/tests/vault-program.ts)

  #### Escrow Program

  This program is an escrow program that allows users to transfer token from one account to another.
  we have the maker and the taker.

  - The maker:
    - deposits token a certain amount of TokenA into the escrow account, the deposited token is send to the vault in the escrow account, and then specified the amount of tokenB to be received.
  - The taker:
    - the taker transfers the specified amount of tokenB to the maker.
    - the taker will receive the deposited tokenA from the vault in the escrow account.
    - if the maker does not have the ATA for the tokenB , it is automatically created. likewise, if the taker does not have the ATA for the tokenA, it is automatically created.
  - Refund:
    - Incase, the maker does not want to swap their token before a taker accepts, the maker can request for refund from the vault and the program will return the token to the maker and then close the vault account.

  Below is the implementation of the program.

  - [Escrow Program](./escrow/programs/escrow/src/lib.rs)
  - [Escrow Test](./escrow/tests/escrow.ts)
  - [Escrow Maker](./escrow/programs/escrow/src/contexts/make.rs)
  - [Escrow Taker](./escrow/programs/escrow/src/contexts/take.rs)
  - [Escrow Redund](./escrow/programs/escrow/src/contexts/refund.rs)

## NFT- Ticket Program

Deployed address on devnet url: https://explorer.solana.com/address/GVpt8Jt7ZeRNccwBThid9eiPRquDP1KiQHs3n3ADdbYn?cluster=devnet
