import bs58 from "bs58";
import promptSync from "prompt-sync";
import fs from "fs";

const prompt = promptSync();

interface Data {
  base58?: string;
}

function walletToBase58() {
  const walletString = prompt("Enter your wallet byte array: ");
  const wallet = new Uint8Array(walletString.split(",").map(Number));
  const base58 = bs58.encode(wallet);

  console.log("Base58 encoded private key:", base58);

  const outputFilePath = "base58_wallet.json";

  let data: Data = {};
  if (fs.existsSync(outputFilePath)) {
    const fileContent = fs.readFileSync(outputFilePath, "utf8");
    data = JSON.parse(fileContent) as Data;
  }

  data.base58 = base58;

  fs.writeFileSync(outputFilePath, JSON.stringify(data, null, 2));
}

walletToBase58();
