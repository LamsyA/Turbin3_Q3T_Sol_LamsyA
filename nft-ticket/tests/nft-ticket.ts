import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program, setProvider, web3 } from "@coral-xyz/anchor";
import { NftTicket } from "../target/types/nft_ticket";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import {
  findMasterEditionPda,
  findMetadataPda,
  MPL_TOKEN_METADATA_PROGRAM_ID,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { publicKey } from "@metaplex-foundation/umi";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

describe("nft-ticket", async () => {
  // Configure the client to use the local cluster.

  const provider = AnchorProvider.env();
  setProvider(provider);
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftTicket as Program<NftTicket>;
  // create event account
  const eventAccount = anchor.web3.Keypair.generate();

  //  ticket price in lamports is set to 1000
  const ticketPrice = 20000;

  const ticketAccountForNft = web3.Keypair.generate();
  const connection = program.provider.connection;

  /**
   * Airdrop 1 SOL to the given address.
   * @param address the address to airdrop to
   */
  const airdropSol = async (address: PublicKey) => {
    const airdropSignature = await connection.requestAirdrop(
      address,
      1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);
    console.log(
      "✅ Transaction successful for maker:",
      `https://explorer.solana.com/tx/${airdropSignature}?cluster=devnet \n`
    );
    // Example usage:
    // await airdropAddress(someWallet.publicKey);
  };

  //  generate oraganizer wallet
  const organizerWallet = anchor.web3.Keypair.generate();

  const eventName = "test event";

  const date = new anchor.BN(
    Math.floor(new Date("2023-09-01T00:00:00.000Z").getTime() / 1000)
  );
  const maxSupply = new anchor.BN(100);
  const description = "test description";
  // create event pda
  const [eventPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("event"), Buffer.from(eventName)],
    program.programId
  );

  const [ticketPda, ticketBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("ticket"), eventPda.toBuffer()],
    program.programId
  );
  it("create event", async () => {
    console.log(`\n ---------------------------------------\n
                      \n ✅ Create event
                  \n ---------------------------------------\n`);

    // airdrop 1 SOL to organizer wallet
    await airdropSol(organizerWallet.publicKey);

    const organizerBalanceBefore = await provider.connection.getBalance(
      organizerWallet.publicKey
    );
    console.log(
      "Organizer balance before:",
      organizerBalanceBefore / LAMPORTS_PER_SOL,
      "SOL"
    );

    console.log("Event PDA:", eventPda.toBase58());
    console.log("Ticket PDA:", ticketPda.toBase58());
    // console.log("Ticket Bump:", ticketBump);

    const tx = await program.methods
      .createEvent(eventName, ticketPrice, date, maxSupply, description)
      .accounts({
        organizer: organizerWallet.publicKey,
        event: eventPda,
        // ticket: ticketPda.,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([organizerWallet])
      .rpc();

    console.log("✅ Transaction successful: ", tx);

    // Fetch the created event to verify
    const eventData = await program.account.event.fetch(eventPda);
    console.log("Event data:", eventData);
    assert.equal(eventData.eventName, eventName);
    assert.equal(eventData.description, description);
    const organizerBalanceAfter = await provider.connection.getBalance(
      organizerWallet.publicKey
    );
    console.log(
      "Organizer balance after:",
      organizerBalanceAfter / LAMPORTS_PER_SOL
    );
  });
  it("create ticket", async () => {
    console.log(`\n ---------------------------------------\n
                      \n ✅ Create Ticket
                  \n ---------------------------------------\n`);

    const tx = await program.methods.createNft().accounts({}).signers([]).rpc();

    console.log("✅ Transaction successful: ", tx);
  });
});
function airdropSol(publicKey: anchor.web3.PublicKey) {
  throw new Error("Function not implemented.");
}
