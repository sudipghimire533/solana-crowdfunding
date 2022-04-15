const instruction = require("./instruction");
const utils = require("./utils");
const path = require("path");
const solana = require("@solana/web3.js");

const PROGRAM_PATH = path.resolve(__dirname, "../target/deploy/");
const PROGRAM_LIB = path.join(PROGRAM_PATH, "crowdfund.so");
const PROGRAM_KEY = path.join(PROGRAM_PATH, "crowdfund-keypair.json");

let programKeypair;
let connection;
let payer;

const SYSTEM_RPOGRAM = solana.SystemProgram.programId;

async function establishConnection() {
    programKeypair = await utils.createKeypairFromFile(PROGRAM_KEY);
    connection = new solana.Connection("http://127.0.0.1:8899", 'confirmed');
    payer = await utils.establishPayer(connection);
}

async function createProjectInstruction(params) {
    let project_name = params.name;
    let project_target = params.target;
    let project_owner = params.owner;

    let bank_seed = [project_owner.toBuffer(), Buffer.from(project_name)];
    let [bank_address, bank_bump] = await solana.PublicKey.findProgramAddress(
        bank_seed,
        programKeypair.publicKey,
    );

    let address_seed = [bank_address.toBuffer(), Buffer.from(bank_bump.toString())];
    let [project_address, project_bump] = await solana.PublicKey.findProgramAddress(
        address_seed, programKeypair.publicKey
    );

    let create_params = new instruction.createParams({
        name: project_name,
        target: project_target,
        project_bump
    });

    let create_project_instruction = Buffer.concat(
        [Buffer.from([instruction.CREATE_PROJECT]), create_params.serialize()],
    );

    return {
        data: create_project_instruction,
        bank_address,
        project_address
    };
}

async function complimentProject(params) {
    let creditor = params.creditor;
    let project_address = params.project;
    let amount = params.amount;

    let projectInfo = await utils.getProjectInfo(connection, project_address);
    let project_bank = projectInfo.bank;

    let compliment_params = new instruction.complimentParams(amount);
    let compliment_project_instruction = Buffer.concat(
        [Buffer.from([instruction.COMPILMENT_PROJECT]), compliment_params.serialize()],
    );

    let compliment_tx = new solana.TransactionInstruction({
        keys: [
            { pubkey: SYSTEM_RPOGRAM, isSigner: false, isWritable: false },
            { pubkey: creditor.publicKey, isSigner: true, isWritable: true },
            { pubkey: project_address, isSigner: false, isWritable: true },
            { pubkey: project_bank, isSigner: false, isWritable: true },
        ],
        programId: programKeypair.publicKey,
        data: compliment_project_instruction,
    });

    let tx_hash = await solana.sendAndConfirmTransaction(
        connection,
        new solana.Transaction().add(compliment_tx),
        [creditor],
    );

    return tx_hash;
}

async function createProject(params) {
    let project_name = params.name;
    let project_target = params.target;
    let project_owner = params.owner;

    let bank_seed = [project_owner.publicKey.toBuffer(), Buffer.from(project_name)];
    let [bank_address, bank_bump] = await solana.PublicKey.findProgramAddress(
        bank_seed,
        programKeypair.publicKey,
    );

    let address_seed = [bank_address.toBuffer(), Buffer.from([bank_bump])];
    let [project_address, project_bump] = await solana.PublicKey.findProgramAddress(
        address_seed, programKeypair.publicKey
    );

    let create_params = new instruction.createParams({
        name: project_name,
        target: project_target,
        project_bump
    });

    let create_project_instruction = Buffer.concat(
        [Buffer.from([instruction.CREATE_PROJECT]), create_params.serialize()],
    );

    let create_tx = new solana.TransactionInstruction({
        keys: [
            { pubkey: SYSTEM_RPOGRAM, isSigner: false, isWritable: false },
            { pubkey: params.owner.publicKey, isSigner: true, isWritable: true },
            { pubkey: bank_address, isSigner: false, isWritable: true },
            { pubkey: project_address, isSigner: false, isWritable: true },
        ],
        programId: programKeypair.publicKey,
        data: create_project_instruction,
    });

    let call_hash = await solana.sendAndConfirmTransaction(
        connection,
        new solana.Transaction().add(create_tx),
        [params.owner],
    );

    return {
        call_hash,
        project_address,
        bank_address,
    };
}

async function main() {
    await establishConnection();

    // Create a user who will be creating a project
    let project_creator = solana.Keypair.generate();
    console.log(`Project creator is ${project_creator.publicKey.toBase58()}`);

    // Fund that creator so can able to call transaction
    await utils.transferFund(connection, {
        from: payer,
        to: project_creator.publicKey,
        amount: solana.LAMPORTS_PER_SOL * 10
    });


    let create_res = await createProject({
        name: "crowdfund",
        target: solana.LAMPORTS_PER_SOL * 10,
        owner: project_creator,
    });
    let crowdfund_address = create_res.project_address;
    console.log("Created project. Hash:", create_res.call_hash);
    console.log("Address:", crowdfund_address.toBase58());
    console.log("Bank:", create_res.bank_address.toBase58());

    // Create anpther user who will donate to the creditor
    // and deposit fund to this new user
    let alice = solana.Keypair.generate();
    console.log("Alice is", alice.publicKey.toBase58());
    await utils.transferFund(connection, {
        from: payer,
        to: alice.publicKey,
        amount: solana.LAMPORTS_PER_SOL * 10,
    });

    let compliment_amount = 2 * solana.LAMPORTS_PER_SOL;
    let compliment_hash = await complimentProject({
        amount: compliment_amount,
        project: crowdfund_address,
        creditor: alice,
    });
    console.log(
        "Alice complimented project with amount of: ",
        compliment_amount / solana.LAMPORTS_PER_SOL,
        ". Call Hash", compliment_hash
    );


}

main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);