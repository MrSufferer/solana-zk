import * as web3 from "@solana/web3.js";
import * as snarkjs from "snarkjs";
import path from "path";
import { buildBn128, utils } from "ffjavascript";
const { unstringifyBigInts } = utils;

const wasmPath = path.join(__dirname, "../circuits", "Multiplier.wasm");
const zkeyPath = path.join(__dirname, "../circuits", "Multiplier_final.zkey");

const PROGRAM_ID = new web3.PublicKey("2guJLwK6P9NzjtjAb2BMr865nw5ExqCHLgonv32duxZw");
const ACCOUNT_TO_QUERY = new web3.PublicKey("511wHA3Q7KV4rNL2WN3LY1t9DCNn5nYVujTU3RgQuHZS");
const connection = new web3.Connection(web3.clusterApiUrl('devnet'), "confirmed");
const wallet = web3.Keypair.generate();

describe('Groth16 Verifier', () => {
  it('should verify a valid proof', async () => {
    // Request an airdrop to fund the wallet
    const airdropSignature = await connection.requestAirdrop(
      wallet.publicKey,
      web3.LAMPORTS_PER_SOL * 1 // Request 1 SOL
    );
    await connection.confirmTransaction(airdropSignature);

    // Generate proof using snarkjs
    let input = { "a": 3, "b": 4 };
    let { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmPath, zkeyPath);

    let curve = await buildBn128();
    let proofProc = unstringifyBigInts(proof);
    publicSignals = unstringifyBigInts(publicSignals);

    let pi_a = g1Uncompressed(curve, proofProc.pi_a);
    pi_a = reverseEndianness(pi_a)
    pi_a = await negateAndSerializeG1(curve, pi_a);
    let pi_a_0_u8_array = Array.from(pi_a);
    console.log(pi_a_0_u8_array);

    const pi_b = g2Uncompressed(curve, proofProc.pi_b);
    let pi_b_0_u8_array = Array.from(pi_b);
    console.log(pi_b_0_u8_array.slice(0, 64));
    console.log(pi_b_0_u8_array.slice(64, 128));

    const pi_c = g1Uncompressed(curve, proofProc.pi_c);
    let pi_c_0_u8_array = Array.from(pi_c);
    console.log(pi_c_0_u8_array);

    // Prepare transaction to send proof to the Solana program
    const transaction = new web3.Transaction();

    transaction.add(web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 }));

    transaction.add(web3.ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 2 }));

    // Define the accounts involved in the transaction
    const accounts = [
      {
        pubkey: wallet.publicKey, // The public key of the account sending the proof
        isSigner: true,
        isWritable: true
      },
      {
        pubkey: ACCOUNT_TO_QUERY,
        isSigner: false,
        isWritable: true
      },
    ];

    // Assuming publicSignals has only one element
    const publicSignalsBuffer = to32ByteBuffer(BigInt(publicSignals[0]));
    let public_signal_0_u8_array = Array.from(publicSignalsBuffer);
    console.log(public_signal_0_u8_array);

    const serializedData = Buffer.concat([
      pi_a,
      pi_b,
      pi_c,
      publicSignalsBuffer
    ]);

      // Create the instruction
      const instruction = new web3.TransactionInstruction({
        keys: accounts,
        programId: PROGRAM_ID,
        data: serializedData // The data containing the proof and public inputs
      });


      // Add the instruction to the transaction
      transaction.add(instruction);

      // Sign and send the transaction
      const signature = await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [wallet], // Array of signers, in this case, just the wallet
        {
          skipPreflight: true
        }
      );

      console.log("Transaction signature", signature);

      // Send and confirm transaction
      await web3.sendAndConfirmTransaction(
        connection,
        transaction,
        [wallet]
      );

      // Fetch and assert the result
      // const ctxRes = await getRes(PROGRAM_ID);
      // expect(ctxRes).not.to.equal(0); // Replace with your expected result
    });
});

function to32ByteBuffer(bigInt) {
  const hexString = bigInt.toString(16).padStart(64, '0'); // Pad to 64 hex characters (32 bytes)
  const buffer = Buffer.from(hexString, "hex");
  return buffer; 
}

function g1Uncompressed(curve, p1Raw) {
  let p1 = curve.G1.fromObject(p1Raw);

  let buff = new Uint8Array(64); // 64 bytes for G1 uncompressed
  curve.G1.toRprUncompressed(buff, 0, p1);

  return Buffer.from(buff);
}

// Function to negate G1 element
function negateG1(curve, buffer) {
  let p1 = curve.G1.fromRprUncompressed(buffer, 0);
  let negatedP1 = curve.G1.neg(p1);
  let negatedBuffer = new Uint8Array(64);
  curve.G1.toRprUncompressed(negatedBuffer, 0, negatedP1);
  return Buffer.from(negatedBuffer);
}

// Function to reverse endianness of a buffer
function reverseEndianness(buffer) {
  return Buffer.from(buffer.reverse());
}

async function negateAndSerializeG1(curve, reversedP1Uncompressed) {
  if (!reversedP1Uncompressed || !(reversedP1Uncompressed instanceof Uint8Array || Buffer.isBuffer(reversedP1Uncompressed))) {
    console.error('Invalid input to negateAndSerializeG1:', reversedP1Uncompressed);
    throw new Error('Invalid input to negateAndSerializeG1');
  }
  // Negate the G1 point
  let p1 = curve.G1.toAffine(curve.G1.fromRprUncompressed(reversedP1Uncompressed, 0));
  let negatedP1 = curve.G1.neg(p1);

  // Serialize the negated point
  // The serialization method depends on your specific library
  let serializedNegatedP1 = new Uint8Array(64); // 32 bytes for x and 32 bytes for y
  curve.G1.toRprUncompressed(serializedNegatedP1, 0, negatedP1);
  // curve.G1.toRprUncompressed(serializedNegatedP1, 32, negatedP1.y);
  console.log(serializedNegatedP1)

  // Change endianness if necessary
  let proof_a = reverseEndianness(serializedNegatedP1);

  return proof_a;
}

function g2Uncompressed(curve, p2Raw) {
  let p2 = curve.G2.fromObject(p2Raw);

  let buff = new Uint8Array(128); // 128 bytes for G2 uncompressed
  curve.G2.toRprUncompressed(buff, 0, p2);

  return Buffer.from(buff);
}