import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultProgram } from "../target/types/vault_program";
import {
  Cluster,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";

describe("vault-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VaultProgram as Program<VaultProgram>;
  const connection = program.provider.connection;
  const state = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.publicKey.toBytes()],
    program.programId
  )[0];

  const bump = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBytes()],
    program.programId
  )[0];

  const user = Keypair.generate();

  const amount = new anchor.BN(1.5 * LAMPORTS_PER_SOL);

  const withdrawalAmount = new anchor.BN(1.5 * LAMPORTS_PER_SOL);
  it("Is initialized!", async () => {
    let airdropSignature = await connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);

    // Check the balance after airdrop
    let balance = await connection.getBalance(user.publicKey);
    console.log(`User balance: ${balance / LAMPORTS_PER_SOL} SOL`);

    console.log(
      "✅ Transaction successful:",
      `https://explorer.solana.com/tx/${airdropSignature}?cluster=devnet`
    );

    const tx = await program.methods
      .initialize()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log(`✅ Transaction successful: ${tx}\n`);
  });

  it("Deposit SOL", async () => {
    const tx = await program.methods
      .deposit(amount)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log(`✅ Transaction successful: ${tx}\n`);

    let balance = await connection.getBalance(user.publicKey);
    console.log(`User balance: ${balance / LAMPORTS_PER_SOL} SOL`);
  });

  it("Withdraw SOL", async () => {
    const tx = await program.methods
      .withdraw(withdrawalAmount)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log(`✅ Transaction successful: ${tx}\n`);

    let balance = await connection.getBalance(user.publicKey);
    console.log(`User balance: ${balance / LAMPORTS_PER_SOL} SOL`);
  });

  it("Close Vault", async () => {
    const tx = await program.methods
      .close()
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();
    console.log(`✅ Transaction successful: ${tx}\n`);

    let balance = await connection.getBalance(user.publicKey);
    console.log(`User balance: ${balance / LAMPORTS_PER_SOL} SOL`);
  });
});
