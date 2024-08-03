import bs58 from "bs58";
import promptSync from "prompt-sync";

const prompt = promptSync();

function base58ToWallet() {
  const base58 = prompt("Enter your base58 encoded private key: ");
  const wallet = bs58.decode(base58);
  console.log("Wallet byte array:", wallet);
}

function walletToBase58() {
  const walletString = prompt("Enter your wallet byte array: ");
  const wallet = new Uint8Array(walletString.split(",").map(Number));
  const base58 = bs58.encode(wallet);
  console.log("Base58 encoded private key:", base58);
}

base58ToWallet();
walletToBase58();
