#!/bin/bash

solana-test-validator --reset -q &
sleep 3
solana config set -u localhost
solana program write-buffer --buffer program_buffer/keypair.json target/deploy/dummy_updated.so
anchor build --skip-lint
anchor deploy --program-name timelock_program
anchor deploy --program-name dummy
anchor test --skip-build --skip-deploy