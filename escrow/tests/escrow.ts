import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import { randomBytes } from "crypto";

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
      `\n ✅ Transaction signature: https://explorer.solana.com/tx/${signature}?cluster=${rpcEndpoint}`
    );
    return signature;
  };
  const tokenProgram = TOKEN_2022_PROGRAM_ID;

  const seed = new anchor.BN(randomBytes(8));

  const [maker, mintA, taker, mintB] = Array.from({ length: 4 }, () =>
    Keypair.generate()
  );

  const [makerAtaA, takerAtaA, makerAtaB, takerAtaB] = [maker, taker].map((k) =>
    [mintA, mintB]
      .map((m) =>
        getAssociatedTokenAddressSync(
          m.publicKey,
          k.publicKey,
          false,
          tokenProgram
        )
      )
      .flat()
  );

  //   [
  //     Buffer.from("escrow"),
  //     maker.publicKey.toBuffer(),
  //     seed.toArrayLike(Buffer, "le", 8),
  //   ],
  //   program.programId
  // )[0];

  const escrow = PublicKey.findProgramAddressSync(
    [
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      mintA.publicKey.toBuffer(),
    ],
    program.programId
  )[0];

  const vault = getAssociatedTokenAddressSync(
    mintA.publicKey,
    maker.publicKey,
    false,
    tokenProgram
  );

  const accounts = {
    maker: maker.publicKey,
    taker: taker.publicKey,
    makerAtaA,
    takerAtaA,
    makerAtaB,
    takerAtaB,
    escrow,
    mintA,
    mintB,
    vault,
    tokenProgram,
    systemProgram: SystemProgram.programId,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  };

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.make().rpc();
    console.log("Your transaction signature", tx);
  });
});
