import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fundraiser } from "../target/types/fundraiser";

describe("fundraiser", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Fundraiser as Program<Fundraiser>;
  const maker = anchor.web3.Keypair.generate();

  let mint;
  let contributorATA;
  let makerATA;
  let wallet;
  const fundraiser = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('fundraiser_seed'),
     maker.publicKey.toBuffer() 
    ],
    program.programId
  )[0];
  const contributor = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('contributer_seed'),
     fundraiser.toBuffer(),
     anchor.AnchorProvider.env().publicKey.toBuffer()
    ],
    program.programId
  )[0];

  console.log("🚀 ~ describe ~ fundraiser:", fundraiser, contributor);

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
  });
});
