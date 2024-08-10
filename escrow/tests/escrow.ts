import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { randomBytes } from "crypto";
import { BN } from "bn.js";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const connection = program.provider.connection;
  const rpcEndpoint = connection.rpcEndpoint;

  const confirm = async (signature: string): Promise<string> => {
    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature,
      ...latestBlockHash,
    });

    return signature;
  };

  const msg = async (signature: string): Promise<string> => {
    console.log(
      `\n ✅ Transaction signature: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
    return signature;
  };

  const seed = new BN(randomBytes(8));
  const amount = new anchor.BN(1000);
  const receive = new anchor.BN(1000);
  const maker = Keypair.generate();
  const taker = Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: PublicKey;
  let makerAtaB: PublicKey;
  let takerAtaA: PublicKey;
  let takerAtaB: PublicKey;
  it("create and mint tokens", async () => {
    console.log(`Escrow seed: ${seed}`);

    let airdropSignature = await connection.requestAirdrop(
      maker.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);

    console.log(
      "✅ Transaction successful for maker:",
      `https://explorer.solana.com/tx/${airdropSignature}?cluster=devnet \n`
    );

    let airdropSignaturefortaker = await connection.requestAirdrop(
      taker.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignaturefortaker);

    console.log(
      "✅ Transaction successful for taker:",
      `https://explorer.solana.com/tx/${airdropSignaturefortaker}?cluster=devnet \n`
    );
    console.log(`\n ✅ Maker: ${maker.publicKey}`);
    console.log(`\n ✅ Taker: ${taker.publicKey}`);

    mintA = await createMint(connection, maker, maker.publicKey, null, 6);
    console.log(`\n ✅ Mint A: ${mintA}`);
    mintB = await createMint(connection, taker, taker.publicKey, null, 6);
    console.log(`\n ✅ Mint B: ${mintB}`);

    makerAtaA = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        maker,
        mintA,
        maker.publicKey
      )
    ).address;
    console.log(`\n ✅ Maker ATA A: ${makerAtaA}`);

    makerAtaB = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        maker,
        mintB,
        maker.publicKey
      )
    ).address;
    console.log(`\n ✅ Maker ATA B: ${makerAtaB}`);

    takerAtaA = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        taker,
        mintA,
        taker.publicKey
      )
    ).address;
    console.log(`\n ✅ Taker ATA A: ${takerAtaA}`);

    takerAtaB = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        taker,
        mintB,
        taker.publicKey
      )
    ).address;
    console.log(`\n ✅ Taker ATA B: ${takerAtaB}`);

    const mintAtoMaker = await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA,
      maker.publicKey,
      1000
    );
    console.log(`\n ✅ Mint A to Maker ${mintAtoMaker}`);

    // const mintBtoMaker = await mintTo(
    //   connection,
    //   taker,
    //   mintB,
    //   makerAtaB,
    //   taker.publicKey,
    //   1000
    // );
    const mintBtoTaker = await mintTo(
      connection,
      taker,
      mintB,
      takerAtaB,
      taker.publicKey,
      1000
    );
    console.log(`\n ✅ Mint B to Taker ${mintBtoTaker}`);

    const escrow = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    const vault = getAssociatedTokenAddressSync(mintA, escrow, true);

    console.log(`\n ✅ Vault: ${vault}`);
    console.log(`\n ✅ Escrow PDA: ${escrow}`);
  });

  it("It should deposit to escrow!", async () => {
    console.log(`\n ---------------------------------------\n
                         \n ✅ Escrow testing starts here
                 \n ---------------------------------------\n`);

    const escrow = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    const vault = getAssociatedTokenAddressSync(mintA, escrow, true);

    const makerTokenABalanceBefore = await connection.getTokenAccountBalance(
      makerAtaA
    );
    console.log(
      `\n ✅ Maker Token A Balance before deposit: ${makerTokenABalanceBefore.value.amount}`
    );
    const accounts = {
      maker: maker.publicKey,
      taker: taker.publicKey,
      mintA: mintA,
      mintB: mintB,
      makerAtaA: makerAtaA,
      takerAtaA: takerAtaA,
      makerAtaB: makerAtaB,
      takerAtaB: takerAtaB,
      escrow,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    };

    const tx = await program.methods
      .make(seed, amount, receive)
      .accounts(accounts)
      .signers([maker])
      .rpc()
      .then(confirm)
      .then(msg);
    console.log("Your transaction signature", tx);

    // check token a balance
    const makerTokenABalance = await connection.getTokenAccountBalance(
      makerAtaA
    );
    console.log(
      `\n ✅ Maker Token A Balance after deposit: ${makerTokenABalance.value.amount}`
    );
    // check token b balance
    const makerTokenBBalance = await connection.getTokenAccountBalance(
      makerAtaB
    );
    console.log(
      `\n ✅ Maker Token B Balance after deposit: ${makerTokenBBalance.value.amount}`
    );
  });

  xit("it should refund the maker", async () => {
    try {
      console.log(`\n ---------------------------------------\n
                         \n ✅ Escrow testing  Refund starts here
                 \n ---------------------------------------\n`);

      // get maker token balance
      const makerTokenABalance = await connection.getTokenAccountBalance(
        makerAtaA
      );
      console.log(
        `\n ✅ Maker Token A Balance before refund: ${makerTokenABalance.value.amount}`
      );
      const escrow = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          seed.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      )[0];
      const vault = getAssociatedTokenAddressSync(mintA, escrow, true);

      const accounts = {
        maker: maker.publicKey,
        taker: taker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA,
        takerAtaA: takerAtaA,
        makerAtaB: makerAtaB,
        takerAtaB: takerAtaB,
        escrow,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      };
      const tx = await program.methods
        .refund()
        .accounts(accounts)
        .signers([maker])
        .rpc()
        .then(confirm)
        .then(msg);
      console.log("Your transaction signature", tx);
    } catch (err) {
      if (err instanceof anchor.web3.SendTransactionError) {
        console.error("Transaction failed with logs:", err.logs);
      } else {
        console.error("An error occurred:", err);
      }
      throw err;
    }

    // check token a balance
    const makerTokenABalance = await connection.getTokenAccountBalance(
      makerAtaA
    );
    console.log(
      `\n ✅ Maker Token A Balance after refund: ${makerTokenABalance.value.amount}`
    );
  });

  it("should Take the escrow!", async () => {
    console.log(`\n -----------------------------------------------------------------------\n
         \n ✅ ------------ Take the escrow starts here ------------
                 \n -----------------------------------------------------------------------\n`);

    const escrow = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    const vault = getAssociatedTokenAddressSync(mintA, escrow, true);

    // Check balances before the take instruction
    const takerTokenBBalanceBefore = await connection.getTokenAccountBalance(
      takerAtaB
    );
    console.log(
      `\n ✅ Taker Token B Balance before take: ${takerTokenBBalanceBefore.value.amount}`
    );

    const makerTokenBBalanceBefore = await connection.getTokenAccountBalance(
      makerAtaB
    );
    console.log(
      `\n ✅ Maker Token B Balance before take: ${makerTokenBBalanceBefore.value.amount}`
    );

    const vaultBalanceBefore = await connection.getTokenAccountBalance(vault);
    console.log(
      `\n ✅ Vault Balance before take: ${vaultBalanceBefore.value.amount}`
    );
    const accounts = {
      maker: maker.publicKey,
      taker: taker.publicKey,
      mintA: mintA,
      mintB: mintB,
      makerAtaA: makerAtaA,
      takerAtaA: takerAtaA,
      makerAtaB: makerAtaB,
      takerAtaB: takerAtaB,
      escrow,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    };

    const tx = await program.methods
      .take()
      .accounts(accounts)
      .signers([taker])
      .rpc()
      .then(confirm)
      .then(msg);
    console.log("Your transaction signature", tx);

    // Check balances after the take instruction
    const takerTokenBBalance = await connection.getTokenAccountBalance(
      takerAtaB
    );
    console.log(
      `\n ✅ Taker Token B Balance after take: ${takerTokenBBalance.value.amount}`
    );
    const makerTokenBBalance = await connection.getTokenAccountBalance(
      makerAtaB
    );
    console.log(
      `\n ✅ Maker Token B Balance after take: ${makerTokenBBalance.value.amount}`
    );
    // check taker balance for token A
    const takerTokenABalance = await connection.getTokenAccountBalance(
      takerAtaA
    );

    console.log(
      `\n ✅ Taker Token A Balance after take: ${takerTokenABalance.value.amount}`
    );
  });
});
