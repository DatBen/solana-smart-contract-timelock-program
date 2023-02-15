import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import { IDL as timeLockIdl } from "../target/types/timelock_program";
import { IDL as dummyIdl } from "../target/types/dummy";
import { timelockProgramId, dummyProgramId, BpfLoaderUpgradable, bufferData } from "./test-config";

describe("timelock_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const timeLockProgram = new Program(timeLockIdl, timelockProgramId, provider);
  const dummyProgram = new Program(dummyIdl, dummyProgramId, provider);

  const timeLock = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("timelock")),
      dummyProgram.programId.toBuffer(),
    ],
    timeLockProgram.programId
  );

  const dummyProgramData = anchor.web3.PublicKey.findProgramAddressSync(
    [dummyProgram.programId.toBuffer()],
    BpfLoaderUpgradable
  );

  timeLockProgram.addEventListener("PlanUpdateEvent", (e) => {
    console.log(e);
  });

  timeLockProgram.addEventListener("CommitUpdateEvent", (e) => {
    console.log(e);
  });

  it("call old dummy", async () => {
    await dummyProgram.methods.initialize().rpc();
  });

  it("init timelock", async () => {
    await timeLockProgram.methods
      .initializeTimeLock(timeLock[1])
      .accounts({
        currentAuthority: provider.publicKey,
        programToBeLocked: dummyProgram.programId,
        programToBeLockedData: dummyProgramData[0],
        timelock: timeLock[0],
        systemProgram: SystemProgram.programId,
        bpfUpgradableLoader: BpfLoaderUpgradable,
      })
      .rpc({ skipPreflight: true });
  });

  it("plan update", async () => {
    await timeLockProgram.methods
      .planUpdate(
        { devOnly: {} },
        "https://github.com/coral-xyz/anchor/tree/v0.26.0"
      )
      .accounts({
        currentAuthority: provider.publicKey,
        lockedProgram: dummyProgram.programId,
        newProgramData: bufferData,
        timelock: timeLock[0],
        systemProgram: SystemProgram.programId,
        bpfUpgradableLoader: BpfLoaderUpgradable,
      })
      .rpc();

    await timeLockProgram.account.timeLock.fetch(timeLock[0]);

    await new Promise((f) => setTimeout(f, 20000));
  });

  it("commit update", async () => {
    await timeLockProgram.methods
      .commitUpdate()
      .accounts({
        timelockAdmin: provider.publicKey,
        lockedProgram: dummyProgram.programId,
        lockedProgramData: dummyProgramData[0],
        newProgramData: bufferData,
        timelock: timeLock[0],
        systemProgram: SystemProgram.programId,
        bpfUpgradableLoader: BpfLoaderUpgradable,
        rent: SYSVAR_RENT_PUBKEY,
        clock: SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();
  });

  it("call new dummy", async () => {
    await dummyProgram.methods.initialize().rpc();
  });
});
