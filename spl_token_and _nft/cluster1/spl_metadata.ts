import wallet from "../wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { base58 } from "@metaplex-foundation/umi/serializers";

// Define our Mint address
const mint = publicKey("DbeVAHAxkxdZm9DnqDi4gbFt7ridkHKWytAroysXZqas");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    // Start here
    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      //   metadata: mint,
      mint: mint,
      mintAuthority: signer,
      payer: signer,
      updateAuthority: signer.publicKey,
    };

    let data: DataV2Args = {
      name: "WBA Token",
      symbol: "WBA",
      uri: "https://arweave.net/9YXO2Y6yDp5P5QHb7TqFf7nO7v2Yt5t0t9t9w2p6ZkY",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data: data,
      isMutable: true,
      collectionDetails: null,
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.deserialize(result.signature);
    console.log(bs58.encode(result.signature));
    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${result.signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
