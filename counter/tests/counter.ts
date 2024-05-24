import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const payer = provider.wallet as anchor.Wallet;

  it("InitializeCounter", async () => {
    const [counterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("counter"))],
      program.programId
    );

    const tx = await program.methods
      .initializeCounter({
        initCount: new BN(10),
      })
      .accounts({
        counter: counterPDA,
      })
      .rpc();

    await program.account.counter.fetch(counterPDA).then((counter) => {
      console.log("Counter initialized to", counter.count.toString());
    });
  });

  it("IncrementCounter", async () => {
    const [counterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("counter"))],
      program.programId
    );

    const tx = await program.methods
      .increaseCount()
      .accounts({
        counter: counterPDA,
      })
      .rpc();

    await program.account.counter.fetch(counterPDA).then((counter) => {
      console.log("Counter increased to", counter.count.toString());
    });
  });
});
