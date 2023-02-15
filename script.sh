#!/bin/bash

solana-test-validator --reset -q &
sleep 3
solana config set -u localhost
solana program write-buffer --buffer program_buffer/keypair.json program_buffer/new_dummy_data.so 
anchor build --skip-lint
anchor deploy
anchor test --skip-build --skip-deploy