import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Program, setProvider, web3 } from "@coral-xyz/anchor";
import { NftTicket } from "../target/types/nft_ticket";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import base58 from "bs58";
import { assert } from "chai";
import { faker, faker as fk } from "@faker-js/faker";
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
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

describe("nft-ticket", async () => {
  // Configure the client to use the local cluster.

  const provider = AnchorProvider.env();
  setProvider(provider);
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftTicket as Program<NftTicket>;
  // create event account
  const eventAccount = anchor.web3.Keypair.generate();
  // create ticket mint account
  const mint = web3.Keypair.generate();
  // create vault account
  const vault = web3.Keypair.generate();
  // create taker account
  const taker = anchor.web3.Keypair.generate();

  const collection_mint = web3.Keypair.generate();

  //   ticket price in lamports is set to 1000
  const ticketPrice = new anchor.BN(0.5 * LAMPORTS_PER_SOL); // 1_000_000_000;

  const connection = program.provider.connection;

  /**
   * Airdrop 1 SOL to the given address.
   * @param address the address to airdrop to
   */
  const airdropSol = async (address: PublicKey) => {
    const airdropSignature = await connection.requestAirdrop(
      address,
      10 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature);
    console.log(
      `✅ Transaction successful for ${address}:`,
      `https://explorer.solana.com/tx/${airdropSignature}?cluster=devnet \n`
    );
    // Example usage:
    // await airdropAddress(someWallet.publicKey);
  };

  //  generate oraganizer wallet
  const organizerWallet = anchor.web3.Keypair.generate();
  const eventName = faker.lorem.words(3); // Generates a random event name with 3 words

  // const eventName = Math.floor(Math.random() * 100000000000).toString();

  const date = new anchor.BN(
    Math.floor(new Date("2023-09-01T00:00:00.000Z").getTime() / 1000)
  );
  const maxSupply = 2;
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
  const [newPda, newBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("authority"), Buffer.from(eventName)],
    program.programId
  );

  const vault_ata = await getAssociatedTokenAddress(
    mint.publicKey,
    vault.publicKey
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

    const tx = await program.methods
      .createEvent(
        eventName,
        ticketPrice,
        date,
        maxSupply,
        description,
        "Testing",
        "https://arweave.net/KuAU7QFkCz20dpyS6umjKHWZGYDKXfrxd76LKd80jW"
      )
      .accounts({
        organizer: organizerWallet.publicKey,
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

    // airdropSol(organizerWallet.publicKey);
    const RPC_ENDPOINT = "http://127.0.0.1:8899";

    const umi = createUmi(RPC_ENDPOINT)
      .use(mplTokenMetadata())
      .use(walletAdapterIdentity(organizerWallet));

    // generate the wallet for the mint NFT

    // Derive the token address account associated with the mint.
    const mint_ata = await getAssociatedTokenAddress(
      mint.publicKey,
      organizerWallet.publicKey
    );
    console.log("Mint Associated token account: ", mint_ata.toBase58());

    // Derive the PDA metadata count.
    const [metadataAccount] = findMetadataPda(umi, {
      mint: publicKey(mint.publicKey),
    });

    // Derive the main PDA edition.
    let masterEditionAccount = findMasterEditionPda(umi, {
      mint: publicKey(mint.publicKey),
    })[0];

    // Check that the ticket exists and does not already have an NFT
    const ticketAccountForNft = web3.Keypair.generate();

    const ticketAccountInfo = await provider.connection.getAccountInfo(
      ticketAccountForNft.publicKey
    );
    console.log("Ticket account info: ", ticketAccountInfo);

    const ticketAccountDataBefore = await program.account.ticket.fetch(
      ticketPda
    );

    console.log(
      "Ticket account data before: ",
      ticketAccountDataBefore.nftMint
    );
    assert.isNull(
      ticketAccountDataBefore.nftMint,
      "The ticket should not have an NFT before creation"
    );

    console.log("Vault ata: ", vault_ata.toBase58());

    // Ensure the vault ATA is created
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        vault.publicKey,
        LAMPORTS_PER_SOL
      )
    );
    const vaultAtaDataBeforeMintingNFT =
      await program.provider.connection.getAccountInfo(vault_ata);
    assert.isNull(vaultAtaDataBeforeMintingNFT);

    const uri =
      "https://arweave.net/KuAU7QFkCz20dpyS6umjKHWZGYDKXfrxd76LKd80jWA";

    // Define metadata information for the NFT.
    const metadata = {
      name: "AlyraSOL",
      symbol: "ASOL",
      uri,
    };
    const txid = await program.methods
      .createCollection()
      .accountsStrict({
        signer: organizerWallet.publicKey,
        vault: vault.publicKey,
        vaultAta: vault_ata,
        mint: mint.publicKey,
        event: eventPda,
        metadataAccount: metadataAccount,
        masterEditionAccount: masterEditionAccount,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
        ticket: ticketPda,
      })
      .signers([organizerWallet, mint, vault])
      .rpc();

    console.log(
      "✅ Transaction successful: ",
      await provider.connection.getSignatureStatus(txid)
    );

    const ticketData = await program.account.ticket.fetch(ticketPda);
    console.log("\n Event data context:", ticketData);

    // Verify the NFT exists by fetching its metadata
    const nftMetadata = await program.provider.connection.getAccountInfo(
      new web3.PublicKey(metadataAccount.toString())
    );
    console.log("NFT metadata: utf-8 ", nftMetadata);
  });

  it("Buy Ticket and mint NFT", async () => {
    console.log(`\n ---------------------------------------\n
                    \n ✅ Buy Ticket and mint NFT
                \n ---------------------------------------\n`);

    // Airdrop 1 SOL to organizer wallet
    await airdropSol(organizerWallet.publicKey);
    await airdropSol(collection_mint.publicKey);
    await airdropSol(taker.publicKey);

    const organizerBalanceBefore = await provider.connection.getBalance(
      organizerWallet.publicKey
    );
    console.log(
      "Organizer balance before:",
      organizerBalanceBefore / LAMPORTS_PER_SOL,
      "SOL"
    );

    const RPC_ENDPOINT = "http://127.0.0.1:8899";

    const umi = createUmi(RPC_ENDPOINT)
      .use(mplTokenMetadata())
      .use(walletAdapterIdentity(organizerWallet));

    // Derive the token address account associated with the mint.
    const collection_ata = await getAssociatedTokenAddress(
      collection_mint.publicKey,
      vault.publicKey
    );
    console.log(
      "Collection Mint Associated token account: ",
      collection_ata.toBase58()
    );

    // Derive the PDA metadata count.
    const [metadataAccount] = findMetadataPda(umi, {
      mint: publicKey(collection_mint.publicKey),
    });

    // Derive the main PDA edition.
    let masterEditionAccount = findMasterEditionPda(umi, {
      mint: publicKey(collection_mint.publicKey),
    })[0];

    const vault_ata = await getAssociatedTokenAddress(
      collection_mint.publicKey,
      vault.publicKey
    );

    // Ensure the vault ATA is created
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        vault.publicKey,
        LAMPORTS_PER_SOL
      )
    );
    const collectionAtaDataBeforeMintingNFT =
      await program.provider.connection.getAccountInfo(collection_ata);
    console.log(
      "Collection ata collection Ata Data Before Minting NFT : ",
      collectionAtaDataBeforeMintingNFT
    );

    const txid = await program.methods
      .createNftForCollection(eventName)
      .accountsStrict({
        taker: taker.publicKey,
        mintAuthority: vault.publicKey,
        destination: vault_ata,
        collectionMint: collection_mint.publicKey,
        event: eventPda,
        metadata: metadataAccount,
        masterEdition: masterEditionAccount,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        ticket: ticketPda,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([taker, collection_mint, vault])
      .rpc();

    console.log(
      "✅ Transaction successful: ",
      await provider.connection.getSignatureStatus(txid)
    );

    const ticketData = await program.account.ticket.fetch(ticketPda);
    console.log("\n Ticket data context:", ticketData);

    // Verify the NFT exists by fetching its metadata
    const nftMetadata = await program.provider.connection.getAccountInfo(
      new web3.PublicKey(metadataAccount.toString())
    );
    console.log("NFT metadata: utf-8 ", nftMetadata);

    const event = await program.account.event.fetch(eventPda);
    console.log("\n Event data context:", event);
    // check the balance of the taker wallet
    const takerBalanceAfter = await provider.connection.getBalance(
      taker.publicKey
    );
    console.log("Taker balance after:", takerBalanceAfter / LAMPORTS_PER_SOL);
  });
});
function airdropSol(publicKey: anchor.web3.PublicKey) {
  throw new Error("Function not implemented.");
}
