import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Nanobook } from "../target/types/nanobook";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("nanobook", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Nanobook as Program<Nanobook>;

  const NANO_MINT = new PublicKey("GqxQxLwtf5v71LWqfo63do6mp4rqfUHuLc9pWygSQA11");
  const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");

  const KEYPAIR_1 = Keypair.generate();
  const KEYPAIR_2 = Keypair.generate();

  console.log(KEYPAIR_1.secretKey);
  console.log(KEYPAIR_2.secretKey);

  const [ORDERBOOK, _obBump] = PublicKey.findProgramAddressSync([Buffer.from('ob')], program.programId);
  const [USERMAP, _umBump] = PublicKey.findProgramAddressSync([Buffer.from('usermap')], program.programId);

  it.skip("Initializes Orderbook!", async () => {
    const ORDERBOOK_NANO = await getAssociatedTokenAddress(NANO_MINT, ORDERBOOK, true);
    const ORDERBOOK_WSOL = await getAssociatedTokenAddress(WSOL_MINT, ORDERBOOK, true);

    await program.methods.initializeOrderbook()
      .accounts({
        book: ORDERBOOK,
        nanoMint: NANO_MINT,
        nanoVault: ORDERBOOK_NANO,
        solMint: WSOL_MINT,
        solVault: ORDERBOOK_WSOL,
        payer: program.provider.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .rpc()
      .then((sig) => console.log(`Successfully initialized orderbook: ${sig}`))
      .catch((err) => console.log(`Failed to initialize orderbook: ${err}`));

    await program.methods.reallocOrderbook(20480)
      .accounts({
        book: ORDERBOOK,
        payer: program.provider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then((sig) => console.log(`Successfully reallocated orderbook: ${sig}`))
      .catch((err) => console.log(`Failed to reallocated orderbook: ${err}`));

    await program.methods.reallocOrderbook(20648)
        .accounts({
          book: ORDERBOOK,
          payer: program.provider.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
        .then((sig) => console.log(`Successfully reallocated orderbook: ${sig}`))
        .catch((err) => console.log(`Failed to reallocated orderbook: ${err}`));

    await program.methods.hydrateOrderbook()
        .accounts({
          book: ORDERBOOK,
          payer: program.provider.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
        .then((sig) => console.log(`Successfully hydrated orderbook: ${sig}`))
        .catch((err) => console.log(`Failed to hydrate orderbook: ${err}`));
  });

  it.skip("Initializes UserMap!", async () => {
    await program.methods.initializeUsermap()
      .accounts({
        usermap: USERMAP,
        payer: program.provider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then((sig) => console.log(`Successfully initialized usermap: ${sig}`))
      .catch((err) => console.log(`Failed to initialize usermap: ${err}`));
  })

  it("Initializes a maker!", async () => {
    await program.methods.initializeUser()
      .accounts({
        usermap: USERMAP,
        payer: KEYPAIR_1.publicKey,
        systemProgram: SystemProgram.programId
      })
      .signers([KEYPAIR_1])
      .rpc()
      .then((sig) => console.log(`Successfully initialized USER1: ${sig}`))
      .catch((err) => console.log(`Failed to initialize USER1: ${err}`));
  })

  it("Initializes a taker!", async () => {
    await program.methods.initializeUser()
      .accounts({
        usermap: USERMAP,
        payer: KEYPAIR_2.publicKey,
        systemProgram: SystemProgram.programId
      })
      .signers([KEYPAIR_2])
      .rpc()
      .then((sig) => console.log(`Successfully initialized USER2: ${sig}`))
      .catch((err) => console.log(`Failed to initialize USER2: ${err}`));
  })
});
