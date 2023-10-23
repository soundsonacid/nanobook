# nanobook

this is the second project in my budding series of ultra-minimal barebones blockchain projects

it is a 100% on-chain CLOB that will be as small as i could possibly make it to still resemble what a CLOB is supposed to do

currently, nanobook runs 812 lines of rust with comments & whitespace - as of 10/22/23 this is before i've written tests for it to make sure everything works as expected

nanobook only supports two markets total - SOL/ANY and ANY/SOL 

because i am writing tests for this with a token i decided to call "NANO", the markets & balances are denoted with Nano in several places for clarity

a few implementation decisions (128-order limit, FreeBitmap, token_utils) came from metaproph3t and https://github.com/metaDAOproject/YALOB

obviously this is NOT be intended for production usage and is more just for fun & to learn about CLOB architecture

one potentially controversial (bad?) decision i made here to keep matching & execution entirely on-chain:

order execution is represented by internal account changes and NOT actual token transfers 

therefore, the Orderbook's base_vault and quote_vault only change total balance on deposit and withdrawal

this introduces the potential for overwithdrawal, which i've guarded against by requiring that a user's withdrawal is lte their corresponding balance