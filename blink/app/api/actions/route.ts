import {
  ActionGetResponse,
  ActionPostRequest,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
} from "@solana/actions";
import { PublicKey, clusterApiUrl, Connection } from "@solana/web3.js";

import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { nftUpload } from "./nft_upload";
import { uploadMetadata } from "./nft_metadata";
import { nftMint } from "./mft_mint";

export const getRandomImageUrl = () => {
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

export const GET = (req: Request) => {
  const payload: ActionGetResponse = {
    icon: new URL("solana5.jpeg", new URL(req.url).origin).toString(),
    label: "Mint some Rug Sol  NFT",
    title: "Solana changing the world",
    description:
      "This is a protocol that mint nfts of how solana will change the world",
  };

  return Response.json(payload, {
    headers: ACTIONS_CORS_HEADERS,
  });
};

export const OPTIONS = GET;

export const POST = async (req: Request) => {
  try {
    const body: ActionPostRequest = await req.json();

    if (!body || !body.account) {
      return Response.json("Non valid payload provided", {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      });
    }

    let account: PublicKey;
    try {
      account = new PublicKey(body.account);
    } catch (error) {
      return Response.json('Invalid "account" provided', {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      });
    }

    const imageUri = await nftUpload();
    if (imageUri === undefined) {
      return Response.json("Failed to upload image", {
        status: 500,
        headers: ACTIONS_CORS_HEADERS,
      });
    }

    const metadata = await uploadMetadata(imageUri);

    const tx = await nftMint(account, metadata);

    const payload = await createPostResponse({
      fields: {
        transaction: tx,
        message: "You got SolRug!",
      },
    });

    return Response.json(payload, {
      headers: ACTIONS_CORS_HEADERS,
    });
  } catch (error) {
    console.error(error);
    return Response.json("InternalServerError" + error, { status: 500 });
  }
};
