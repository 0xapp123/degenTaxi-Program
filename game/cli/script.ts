import { Program, web3 } from '@project-serum/anchor';
import * as anchor from '@project-serum/anchor';
import {
    LAMPORTS_PER_SOL,
} from '@solana/web3.js';

import { IDL as GameIDL } from "../target/types/game";
import {
    Keypair,
    PublicKey,
    Connection,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    Transaction,
} from '@solana/web3.js';
import {
    GAME_PROGRAM_ID, VAULT_SEED,
} from './types';

let program: Program = null;

// Address of the deployed program.
let programId = new anchor.web3.PublicKey(GAME_PROGRAM_ID);

anchor.setProvider(anchor.AnchorProvider.local(web3.clusterApiUrl("devnet")));
let provider = anchor.getProvider();

const solConnection = anchor.getProvider().connection;
const payer = anchor.AnchorProvider.local().wallet;

// Generate the program client from IDL.
program = new anchor.Program(GameIDL as anchor.Idl, programId);
console.log('ProgramId: ', program.programId.toBase58());

const main = async () => {
    

   
    // await initProject();

    // await deposit(0.5);
    await withdraw(new PublicKey('32NL69SFk8GLPFZfKQwsuexcXHd7rqAQn1mrasF1ksVj'), 0.1);
};

export const initProject = async (
) => {
    const tx = await createInitializeTx(payer.publicKey, program);
    const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
    });
    console.log("txHash =", txId);
}

export const deposit = async (
    amount: number
) => {
    const tx = await createDepositTx(payer.publicKey, amount, program);
    const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
    });
    console.log("txHash =", txId);
}
    

export const withdraw = async (
    receiver: PublicKey,
    amount: number
) => {
    const tx = await createWithdrawTx(payer.publicKey, receiver, amount, program);
    const txId = await provider.sendAndConfirm(tx, [], {
        commitment: "confirmed",
    });
    console.log("txHash =", txId);
}


export const createInitializeTx = async (
    userAddress: PublicKey,
    program: anchor.Program,
) => {
    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        GAME_PROGRAM_ID,
    );
    
    let tx = new Transaction();
    console.log('==>initializing program');

    tx.add(program.instruction.initialize(
        {
        accounts: {
            admin: userAddress,
            solVault,
            systemProgram: SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        },
        instructions: [],
        signers: [],
    }));

    return tx;
}

export const createDepositTx = async (
    userAddress: PublicKey,
    amount: number,
    program: anchor.Program,
) => {
    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        GAME_PROGRAM_ID,
    );

    let tx = new Transaction();
    
    console.log('==> depositing ... ' );

    tx.add(program.instruction.desposit(
        new anchor.BN(amount*LAMPORTS_PER_SOL), {
        accounts: {
            admin: userAddress,
            solVault,
            systemProgram: SystemProgram.programId,
        },
        instructions: [],
        signers: [],
    }));

    return tx;
}


export const createWithdrawTx = async (
    userAddress: PublicKey,
    receiver: PublicKey,
    amount: number,
    program: anchor.Program,
) => {
    const [solVault, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(VAULT_SEED)],
        GAME_PROGRAM_ID,
    );

    let tx = new Transaction();
    
    console.log('==> withdrawing ... ' );

    tx.add(program.instruction.withdraw(
        new anchor.BN(amount*LAMPORTS_PER_SOL), bump, {
        accounts: {
            admin: userAddress,
            receiver,
            solVault,
            systemProgram: SystemProgram.programId,
        },
        instructions: [],
        signers: [],
    }));

    return tx;
}

main();
