# Solana Rust Program with Anchor IDL

This project is a Rust-based CLI tool and test suite designed to interact with a Solana program using an Anchor IDL. It provides functionalities such as:

- Generating Solana keypairs
- Converting between base58 encoded private keys and wallet byte arrays
- Requesting SOL airdrops
- Transferring SOL
- Enrolling in a Solana program

## Project Structure

- **src/programs/mod.rs**: Declares the `wba_prereq` module.
- **src/programs/wba_prereq.rs**: Uses the `solana-idlgen` macro to generate Rust code from the provided IDL.
- **src/lib.rs**: Includes the main test suite to demonstrate the project's functionalities.

## Solana Program IDL

The project uses the following Solana program IDL to interact with the program on Devnet:

```json
{
  "version": "0.1.0",
  "name": "wba_prereq",
  "instructions": [
    {
      "name": "complete",
      "accounts": [
        { "name": "signer", "isMut": true, "isSigner": true },
        { "name": "prereq", "isMut": true, "isSigner": false },
        { "name": "systemProgram", "isMut": false, "isSigner": false }
      ],
      "args": [{ "name": "github", "type": "bytes" }]
    },
    {
      "name": "update",
      "accounts": [
        { "name": "signer", "isMut": true, "isSigner": true },
        { "name": "prereq", "isMut": true, "isSigner": false },
        { "name": "systemProgram", "isMut": false, "isSigner": false }
      ],
      "args": [{ "name": "github", "type": "bytes" }]
    }
  ],
  "accounts": [
    {
      "name": "PrereqAccount",
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "github", "type": "bytes" },
          { "name": "key", "type": "publicKey" }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidGithubAccount",
      "msg": "Invalid Github account"
    }
  ],
  "metadata": {
    "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
  }
}
```

### Success! Check out your TX [here:](https://explorer.solana.com/tx/4KsX4kFNXJbyjCxrZ6bvtXxtr52Ype7WeabKdobHYY4ZfTRE8BroKty1y4mcvAaAVd9RAQCtgubegLwJYQhEnEYH/?cluster=devnet)

## License

This project is licensed under the MIT License
