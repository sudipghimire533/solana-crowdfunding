const fs = require("mz/fs");
const path = require("path");
const os = require("os");
const solana = require("@solana/web3.js");

async function createKeypairFromFile(filePath) {
    const secretKeyString = await fs.readFile(filePath, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));

    return solana.Keypair.fromSecretKey(secretKey);
}

async function establishPayer(connection) {
    const PAYER_KEY = path.join(os.homedir(), ".config/solana/id.json");
    const payer = await createKeypairFromFile(PAYER_KEY);

    let airdrop_request = await connection.requestAirdrop(payer.publicKey, 10 * solana.LAMPORTS_PER_SOL);
    connection.confirmTransaction(airdrop_request);

    return payer;
}

async function transferFund(connection, params) {
    let tx_call = solana.SystemProgram.transfer({
        fromPubkey: params.from.publicKey,
        toPubkey: params.to,
        lamports: params.amount,
    });

    let tx_hash = await solana.sendAndConfirmTransaction(
        connection,
        new solana.Transaction().add(tx_call),
        [params.from]
    );

    return tx_hash;
}

exports.transferFund = transferFund;
exports.establishPayer = establishPayer;
exports.createKeypairFromFile = createKeypairFromFile;