import bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

export const getWallet = () => {
  const privkey = process.env.WALLET;

  if (!privkey) {
    throw new Error("WALLET is missing");
  }
  const wallet = bs58.decode(privkey as string);

  return wallet;
};
