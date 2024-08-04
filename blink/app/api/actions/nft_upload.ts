import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

import { getWallet } from "./wallet";
import { readFile } from "fs/promises";
import path from "path";

const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(getWallet()));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

export const generateRandImgUrl = () => {
  const images = [
    "solana1.jpeg",
    "solana2.jpeg",
    "solana3.jpeg",
    "solana4.jpeg",
    "solana5.jpeg",
    "solana6.jpeg",
    "solana7.jpeg",
    "solana8.jpeg",
    "solana9.jpeg",
    "solana10.jpeg",
  ];
  const randomIndex = Math.floor(Math.random() * images.length);
  return images[randomIndex];
};
export const nftUpload = async () => {
  try {
    //1. Load image

    const getImage = generateRandImgUrl();
    //2. Convert image to generic file.
    const imageFile = await readFile(
      path.join(process.cwd(), "public", getImage)
    );
    const umiImageFile = createGenericFile(imageFile, "RUGGED!", {
      tags: [{ name: "Content-Type", value: "image/jpeg" }],
    });
    //3. Upload image

    const [imageUri] = await umi.uploader
      .upload([umiImageFile])
      .catch((err) => {
        throw new Error(err);
      });
    console.log(" imageUri", imageUri);
    return imageUri;
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
    throw error;
  }
};
