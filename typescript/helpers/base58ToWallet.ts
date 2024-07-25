import bs58 from "bs58";
import promptSync from "prompt-sync";
import fs from "fs";

const prompt = promptSync();

interface Data {
  wallet?: number[];
}

function base58ToWallet() {
  const base58 = prompt("Enter your base58 encoded private key: ");
  const wallet = bs58.decode(base58);

  console.log("Wallet byte array:", wallet);

  const outputFilePath = "wallet.json";

  let data: Data = {};
  if (fs.existsSync(outputFilePath)) {
    const fileContent = fs.readFileSync(outputFilePath, "utf8");
    data = JSON.parse(fileContent) as Data;
  }

  data.wallet = Array.from(wallet);

  fs.writeFileSync(outputFilePath, JSON.stringify(data, null, 2));
}

base58ToWallet();
