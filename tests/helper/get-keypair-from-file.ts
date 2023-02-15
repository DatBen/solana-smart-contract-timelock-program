import { Keypair } from "@solana/web3.js";
import fs from "fs";

export function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(path, "utf-8")))
  );
}
