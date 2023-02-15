import { PublicKey } from "@solana/web3.js";
import { createKeypairFromFile } from "./helper/get-keypair-from-file";

export const BpfLoaderUpgradable = new PublicKey(
  "BPFLoaderUpgradeab1e11111111111111111111111"
);

export const bufferData = createKeypairFromFile(
  "program_buffer/keypair.json"
).publicKey;

export const dummyProgramId = createKeypairFromFile(
  "target/deploy/dummy-keypair.json"
).publicKey;

export const timelockProgramId = createKeypairFromFile(
  "target/deploy/timelock_program-keypair.json"
).publicKey;
