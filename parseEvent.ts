import { Program } from "@project-serum/anchor";

import { PublicKey } from "@solana/web3.js";

import { IDL as timeLockIdl } from "./target/types/timelock_program";

const timelockProgramId = new PublicKey(
  "7tKeFVr5wPggkfQKwTqxt9P6Cd6cRRyMttjj3kT2bjFY"
);
const LOG =
  "UPVj4pyrEUmo/AULGpIqK1/Q1qCszOux+F7jJOSCx1N1/Vv8Sn0IiS4s5mMAAAAAcWpQkpKZH8fZxMc4G9sgSEf86Bi6n9swShdnosr2QRkwAAAAaHR0cHM6Ly9naXRodWIuY29tL2NvcmFsLXh5ei9hbmNob3IvdHJlZS92MC4yNi4w";

const timeLockProgram = new Program(timeLockIdl, timelockProgramId, undefined);

const event = timeLockProgram.coder.events.decode(LOG);

console.log("Event  = ", event);
