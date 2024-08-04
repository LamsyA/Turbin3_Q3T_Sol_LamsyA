import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
  createNoopSigner,
  publicKey,
  Instruction,
} from "@metaplex-foundation/umi";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  toWeb3JsInstruction,
  toWeb3JsKeypair,
} from "@metaplex-foundation/umi-web3js-adapters";
import base58 from "bs58";
import {
  PublicKey,
  TransactionInstruction,
  TransactionMessage,
  VersionedMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { mplCore } from "@metaplex-foundation/mpl-core";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT)
  .use(mplCore())
  .use(mplTokenMetadata())
  .use(irysUploader());

export const nftMint = async (
  account: PublicKey,
  uriMetadata: string
): Promise<VersionedTransaction> => {
  try {
    const signer = createNoopSigner(publicKey(account));
    const mint = generateSigner(umi);
    umi.use(signerIdentity(signer));

    const blockhash = (await umi.rpc.getLatestBlockhash()).blockhash;

    let tx = createNft(umi, {
      mint,
      name: "SOl NFT",
      symbol: "SFT",
      uri: uriMetadata,
      sellerFeeBasisPoints: percentAmount(8.5),
    });

    const createdNftInstructions: Instruction[] = tx.getInstructions();
    const solanaInstructions: TransactionInstruction[] =
      createdNftInstructions.map((ix) => toWeb3JsInstruction(ix));
    const newVersionedmessage: VersionedMessage = new TransactionMessage({
      payerKey: account,
      recentBlockhash: blockhash,
      instructions: solanaInstructions,
    }).compileToV0Message();

    const newTx = new VersionedTransaction(newVersionedmessage);
    const mintKeypair = toWeb3JsKeypair(mint);
    newTx.sign([mintKeypair]);

    return newTx;
  } catch (error) {
    throw error;
  }
};
