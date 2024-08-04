import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { getWallet } from "./wallet";

const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(getWallet()));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const getRandomDiscovery = (): string =>
  `Tech Discovery ${Math.floor(Math.random() * 100) + 1}`;
const getRandomCategory = (): string => {
  const categories = [
    "AI",
    "Blockchain",
    "Quantum Computing",
    "Biotech",
    "Solana",
  ];
  return categories[Math.floor(Math.random() * categories.length)];
};

interface Metadata {
  name: string;
  symbol: string;
  description: string;
  image: string;
  attributes: Array<{ trait_type: string; value: string }>;
  properties: {
    files: Array<{ type: string; uri: string }>;
  };
  creators: Array<any>;
}

export const uploadMetadata = async (imageUri: string): Promise<string> => {
  try {
    const metadata: Metadata = {
      name: "Solana Tech Discovery",
      symbol: "TECH",
      description: "Explore cutting-edge tech discoveries!",
      image: imageUri,
      attributes: [
        { trait_type: "discovery", value: getRandomDiscovery() },
        { trait_type: "category", value: getRandomCategory() },
      ],
      properties: {
        files: [
          {
            type: "image/jpeg",
            uri: imageUri,
          },
        ],
      },
      creators: [],
    };

    return await umi.uploader.uploadJson(metadata);
  } catch (error) {
    throw error;
  }
};
