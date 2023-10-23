# nanobook

this is the second project in my budding series of ultra-minimal barebones blockchain projects

it is a 100% on-chain CLOB that will be as small as i could possibly make it to still resemble what a CLOB is supposed to do

a few implementation decisions (128-order limit, FreeBitmap, token_utils) came from metaproph3t and https://github.com/metaDAOproject/YALOB

obviously this is NOT be intended for production usage and is more just for fun & to learn about CLOB architecture

one potentially controversial (bad?) decision i made here to keep matching & execution entirely on-chain:

order execution is represented by internal account changes and NOT actual token transfers 

therefore, the Orderbook's base_vault and quote_vault only change total balance on deposit and withdrawal

this introduces the potential for overwithdrawal, which we've guarded against by requiring that a user's withdrawal is lte their corresponding balance