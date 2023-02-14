#!/bin/bash

solana config set -u localhost
solana program write-buffer --buffer program_buffer/keypair.json program_buffer/new_dummy_data.so 
solana program set-buffer-authority 8dj6X3kYpKnUXroL7HRcTbsULCYHi9wVbsFAES2jruct --new-buffer-authority 58E3JTEHRTTU4k5f3FoWhBbkgEn11TGVtKwwQYp7r1Az
anchor build --skip-lint
anchor deploy
anchor test --skip-build --skip-deploy