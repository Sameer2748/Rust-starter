import * as borsh from "borsh";
import {expect, test} from "bun:test";
import {Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction} from '@solana/web3.js'
import { DecrementInstruction, IncrementInstruction, instructionSchema, schema, Size } from "./type";

let adminAccount = Keypair.generate();
let counterdataAccount = Keypair.generate();
const connection = new Connection("http://127.0.0.1:8899");
const programId = new PublicKey("G3e8eedonra7JxV4EwNZsxiuHQf9erryfxhwpaG5vyMi");

test("Accoount is initialised", async()=>{

    // drop some solana 
    const tx = await connection.requestAirdrop(adminAccount.publicKey, 1 * LAMPORTS_PER_SOL);
    // confirm the transaction
    await connection.confirmTransaction(tx);
    // get the details of the account the balance
    let details = await connection.getAccountInfo(adminAccount.publicKey);
    console.log(details);


    // now lets start with the dsata account anf give permission to the program id 
    const lamports = await connection.getMinimumBalanceForRentExemption(Size);

    const instruction  = await SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        lamports,
        space:Size,
        programId,
        newAccountPubkey: counterdataAccount.publicKey
    })
    const txn = new Transaction();
    txn.add(instruction);
    // assiged the data account to the program  id  and it have some space 4 byte
    const signature  = await connection.sendTransaction(txn, [adminAccount, counterdataAccount]);

    await connection.confirmTransaction(signature);
    // details of the data accont 
    let dataAccountdetails = await connection.getAccountInfo(counterdataAccount.publicKey);
    console.log(dataAccountdetails)

    // deserialize tje count of this data account details 
    const counter = borsh.deserialize(schema, dataAccountdetails?.data);
    console.log(counter.count);
    expect(counter.count).toBe(0);
})

test("Test increment count", async () => {
    // Create an increment instruction instance, e.g. increment by 1
    const incInstruction = new IncrementInstruction({ value: 22 });
    
    // Cast the Map to the expected borsh.Schema type to fix type errors
    const incrementSchema = instructionSchema as borsh.Schema;
    
    const serializedData = borsh.serialize(incrementSchema, incInstruction);
  
    // Create a transaction instruction targeting our program and counter data account
    const txnInstruction = new TransactionInstruction({
        // acoounts array 
      keys: [
        {
          pubkey: counterdataAccount.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      programId,
      // insstuctions array 
      data: Buffer.from(serializedData),
    });
  
    // Send the transaction containing our increment instruction
    const txn = new Transaction().add(txnInstruction);
    const signature = await connection.sendTransaction(txn, [adminAccount]);
    await connection.confirmTransaction(signature);
    console.log("Increment transaction confirmed:", signature);
  
    // Fetch the updated account data and deserialize to verify the counter has increased
    const updatedAccountInfo = await connection.getAccountInfo(counterdataAccount.publicKey);
    const counter = borsh.deserialize(schema, updatedAccountInfo!.data);
    console.log("Updated counter:", counter.count);
    expect(counter.count).toBe(22);
  });

test("Test decrement count", async () => {
    // Create an increment instruction instance, e.g. increment by 1
    const decInstruction = new DecrementInstruction({ value: 10 });
    
    // Cast the Map to the expected borsh.Schema type to fix type errors
    const decrementSchema = instructionSchema as borsh.Schema;
    
    const serializedData = borsh.serialize(decrementSchema, decInstruction);
  
    // Create a transaction instruction targeting our program and counter data account
    const txnInstruction = new TransactionInstruction({
        // acoounts array 
      keys: [
        {
          pubkey: counterdataAccount.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      programId,
      // insstuctions array 
      data: Buffer.from(serializedData),
    });
  
    // Send the transaction containing our increment instruction
    const txn = new Transaction().add(txnInstruction);
    const signature = await connection.sendTransaction(txn, [adminAccount]);
    await connection.confirmTransaction(signature);
    console.log("Increment transaction confirmed:", signature);
  
    // Fetch the updated account data and deserialize to verify the counter has increased
    const updatedAccountInfo = await connection.getAccountInfo(counterdataAccount.publicKey);
    const counter = borsh.deserialize(schema, updatedAccountInfo!.data);
    console.log("Updated counter:", counter.count);
    expect(counter.count).toBe(12);
  });

