import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    const [counterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("counter"))],
      program.programId
    );

    const tx = await program.methods
      .initializeCounter()
      .accounts({
        counter: counterPDA,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    await program.account.counter.fetch(counterPDA).then((counter) => {
      console.log("Counter initialized to: ", counter.count.toNumber());
      expect(counter.count.toNumber()).equals(0);
    });
  });

  it("Can increment!", async () => {
    const [counterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("counter"))],
      program.programId
    );

    await program.methods
      .increaseCounter()
      .accounts({
        counter: counterPDA,
      })
      .rpc();

    await program.account.counter.fetch(counterPDA).then((counter) => {
      console.log("Counter incremented to: ", counter.count.toNumber());
      expect(counter.count.toNumber()).equals(1);
    });
  });
});
