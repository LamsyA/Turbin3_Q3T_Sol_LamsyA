# Vault Program

## Overview

This README provides an overview of the Vault Program written in Rust using the Anchor framework for the Solana blockchain. The Vault Program allows users to create a vault, deposit funds, withdraw funds, and close the vault, transferring any remaining balance back to the user.

## Program Structure

The program consists of four main instructions:

1. **Initialize**: Sets up the vault and the associated state account for the user.
2. **Deposit**: Allows the user to deposit SOL into their vault.
3. **Withdraw**: Allows the user to withdraw SOL from their vault.
4. **Close**: Closes the vault, transferring any remaining funds back to the user.

## Instructions

### Initialize

The `initialize` function sets up the vault state and the vault account. It requires the user's signature and creates a new vault for the user.

- **Accounts:**
  - `user`: The signer initializing the vault.
  - `vault_state`: The state account storing the vault's metadata.
  - `vault`: The SystemAccount that will hold the deposited SOL.
  - `system_program`: The System program on Solana.

### Deposit

The `deposit` function allows the user to deposit a specified amount of SOL into their vault.

- **Accounts:**
  - `user`: The signer making the deposit.
  - `vault_state`: The state account associated with the user's vault.
  - `vault`: The vault account that will receive the SOL.
  - `system_program`: The System program on Solana.

### Withdraw

The `withdraw` function allows the user to withdraw a specified amount of SOL from their vault.

- **Accounts:**
  - `user`: The signer making the withdrawal.
  - `vault_state`: The state account associated with the user's vault.
  - `vault`: The vault account from which the SOL will be withdrawn.
  - `system_program`: The System program on Solana.

### Close

The `close` function closes the vault, transferring any remaining balance in the vault back to the user. The vault state account is also closed.

- **Accounts:**
  - `user`: The signer closing the vault.
  - `vault_state`: The state account associated with the user's vault.
  - `vault`: The vault account holding the remaining SOL.
  - `system_program`: The System program on Solana.

## Data Structures

### VaultState

The `VaultState` struct stores the metadata for each vault, including the bumps used for PDA (Program Derived Address) calculations.

- **Fields:**
  - `vault_bump`: The bump seed for the vault PDA.
  - `state_bump`: The bump seed for the vault state PDA.

## Implementation Details

- **Program ID:** The program is declared with the ID `5dRpMa9LXDuAjdsyeWxT7UHfTHPZxEXUacnvem3ouEWA`.
- **PDAs:** The program uses Program Derived Addresses (PDAs) for the vault and state accounts. The seeds used include a static string (`"state"` or `"vault"`) and the user's public key.

## Example Usage

To use this program, you will need to:

1. **Initialize** a vault:

   ```rust
   vault_program::initialize(ctx);
   ```

2. **Deposit** SOL into the vault:

   ```rust
   vault_program::deposit(ctx, amount);
   ```

3. **Withdraw** SOL from the vault:

   ```rust
   vault_program::withdraw(ctx, amount);
   ```

4. **Close** the vault:

   ```rust
   vault_program::close(ctx);
   ```

   <br/>

### User Guide: Using the Vault Program on Solana

### Overview

The Vault Program on Solana allows you to securely deposit and withdraw SOL (Solana's native cryptocurrency) from a personal vault, which you can create, manage, and close at any time. This guide will walk you through the process of using the Vault Program from a user perspective.

### Prerequisites

Before you begin, ensure that you have:

1. **A Solana Wallet**: You'll need a wallet like Phantom, Sollet, or any other Solana-compatible wallet.
2. **SOL Tokens**: Make sure you have some SOL tokens in your wallet to interact with the program.
3. **Connection to the Solana Network**: You can use either the mainnet or a testnet like Devnet.

### Setting Up Your Vault

#### Step 1: Initialize the Vault

The first step is to create your vault. When you initialize the vault, a vault account is created under your control.

- **Action**: Send an `initialize` transaction using your wallet.
- **Outcome**: A vault account and a state account are created. The vault is ready to receive deposits.

#### Step 2: Deposit SOL into Your Vault

Once your vault is initialized, you can deposit SOL into it.

- **Action**: Send a `deposit` transaction with the amount of SOL you want to deposit.
- **Outcome**: The specified amount of SOL is transferred from your wallet to your vault account.

#### Step 3: Withdraw SOL from Your Vault

You can withdraw SOL from your vault at any time.

- **Action**: Send a `withdraw` transaction with the amount of SOL you want to withdraw.
- **Outcome**: The specified amount of SOL is transferred from your vault account back to your wallet.

### Managing Your Vault

#### Checking Your Vault Balance

To check how much SOL is in your vault, you can view the balance of your vault account using a Solana explorer or directly through your wallet interface if it supports custom accounts.

#### Closing Your Vault

If you no longer need the vault, you can close it, and any remaining SOL in the vault will be returned to your wallet.

- **Action**: Send a `close` transaction.
- **Outcome**: Your vault account is closed, and any remaining balance is transferred back to your wallet. The state account is also closed.

## Example Scenarios

### Scenario 1: Saving SOL for Future Use

You can use the Vault Program to save SOL for future use. By depositing SOL into your vault, you ensure that these funds are stored securely and can only be accessed through the program's controlled withdrawal process.

### Scenario 2: Transferring SOL Between Wallets

If you need to move SOL between different wallets, you can use the Vault Program to facilitate this. Deposit SOL from one wallet into the vault and then withdraw it into another wallet that you control.

### Scenario 3: Simplifying Multiple Transactions

If you frequently perform multiple transactions, you can deposit a lump sum of SOL into your vault and withdraw smaller amounts as needed. This approach minimizes the number of on-chain transactions you need to make from your main wallet, potentially saving on transaction fees.

## Tips and Best Practices

- **Keep Track of Your Transactions**: Use a Solana explorer to monitor your vault’s activity.
- **Secure Your Wallet**: Always ensure that your wallet is secure and that you're aware of any permissions granted to dApps or programs.
- **Understand the Costs**: Be aware of the transaction fees on the Solana network, especially when performing multiple transactions.

## Troubleshooting

- **Insufficient Funds**: If a transaction fails, ensure you have enough SOL in your wallet to cover both the transaction amount and the network fees.
- **Transaction Errors**: If you encounter errors, double-check that you are using the correct accounts and that the vault has been properly initialized.

## Conclusion

The Vault Program is a flexible tool for managing your SOL on the Solana blockchain. Whether you're saving for future use, simplifying transactions, or securely transferring funds, this program provides a user-friendly and secure way to manage your SOL.
