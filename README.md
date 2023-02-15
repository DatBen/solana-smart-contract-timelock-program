# solana-smart-contract-timelock-program

A timelock program for upgradable solana smart contract

Most of programs on solana are now upgradable which can be really usefull,
however i think that it shifts the third party trust to the authority of the program even
if it is a multi-sig.

I find it interesting (even necessary) to delegate the authority of the program to another program that is not upgradable and thus, to be able to set up a system of update timelocked.
It also allows you to set up easily an alert system to monitor the updates of the programs on which you have funds,
since each update plan will publish an event topic that can be easily listened to.

## Install

yarn install

## Run Test

./test.sh
