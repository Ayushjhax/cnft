import * as fs from 'fs';
import { parse } from 'csv-parse';

type CsvRow = { [key: string]: string }; // Définir un type pour les lignes du CSV

/**
 * Lit un fichier CSV et le transforme en un tableau d'objets.
 * @param filePath - Le chemin du fichier CSV à lire.
 * @returns Une promesse qui résout un tableau d'objets représentant chaque ligne du CSV.
 */
export async function readCsv(filePath: string): Promise<CsvRow[]> {
    return new Promise((resolve, reject) => {
        const rows: CsvRow[] = [];

        fs.createReadStream(filePath)
            .pipe(parse({ columns: true, trim: true })) // Utilisation de l'option columns: true pour convertir chaque ligne en objet
            .on('data', (row: CsvRow) => {
                rows.push(row);
            })
            .on('end', () => {
                resolve(rows);
            })
            .on('error', (error: any) => {
                reject(error);
            });
    });
}

import {
	Connection,
	Keypair,
	PublicKey,
	type Signer,
	TransactionInstruction,
	TransactionMessage,
	VersionedTransaction
} from "@solana/web3.js";


export function loadWalletKey(publik:string): Keypair {
	const fs       = require("fs")
	const fileName = `./keypair/${publik}.json`;

	return Keypair.fromSecretKey(
		new Uint8Array(JSON.parse(fs.readFileSync(fileName).toString())),
	);
}

export async function sendVersionedTx(
	connection  : Connection,
	instructions: TransactionInstruction[],
	payer       : PublicKey,
	signers     : Signer[]
) {
	let latestBlockhash = await connection.getLatestBlockhash()

	const messageLegacy = new TransactionMessage({
		payerKey       : payer,
		recentBlockhash: latestBlockhash.blockhash,
		instructions,
	}).compileToLegacyMessage();

	const transation = new VersionedTransaction(messageLegacy)
	transation.sign(signers);
	const signature = await connection.sendTransaction(transation);

	return signature;
}

export function txToLink(signature: string, cluster: string) {
	return "https://explorer.solana.com/tx/"+signature+"?cluster="+cluster;
}

export function addrToLink(signature: string, cluster: string) {
	return "https://explorer.solana.com/address/"+signature+"?cluster="+cluster;
}
