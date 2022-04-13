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

const SYSTEM_RPOGRAM = new solana.PublicKey("11111111111111111111111111111111");

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

    let address_seed = [bank_address.toBuffer()];
    let [project_address, project_bump] = await solana.PublicKey.findProgramAddress(
        address_seed, programKeypair.publicKey
    );

    let create_params = new instruction.createParams({
        bank: bank_address.toBuffer(),
        name: project_name,
        owner: project_owner.toBuffer(),
        target: project_target,
        address: project_address.toBuffer(),
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

async function createProject(params) {
    let { data, bank_address, project_address } = await createProjectInstruction({
        name: params.name,
        target: params.target,
        owner: params.owner.publicKey,
    });

    console.log(`Project is at ${project_address} funded through ${bank_address}`);

    let init_instruction = new solana.TransactionInstruction({
        keys: [
            { pubkey: SYSTEM_RPOGRAM, isSigner: false, isWritable: false },
            { pubkey: params.owner.publicKey, isSigner: true, isWriteable: true },
            { pubkey: bank_address, isSigner: false, isWriteable: true },
            { pubkey: project_address, isSigner: false, isWriteable: true },
        ],
        programId: programKeypair.publicKey,
        data: data,
    });

    let tx_hash = await solana.sendAndConfirmTransaction(
        connection,
        new solana.Transaction().add(init_instruction),
        [params.owner],
    );

    return tx_hash;
}

async function main() {
    await establishConnection();

    let project_creator = solana.Keypair.generate();
    console.log(`Project creator is ${project_creator.publicKey.toBase58()}`);

    await utils.transferFund(connection, {
        from: payer,
        to: project_creator.publicKey,
        amount: solana.LAMPORTS_PER_SOL * 10
    });

    let create_hash = await createProject({
        name: "crowdfund",
        target: solana.LAMPORTS_PER_SOL * 10,
        owner: project_creator,
    });
    console.log("Created project. Hash:", create_hash);
}

main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);