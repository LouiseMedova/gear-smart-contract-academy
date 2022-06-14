import { GearApi, GearKeyring, getWasmMetadata } from '@gear-js/api';
import { readFileSync } from 'fs';
import {config } from 'dotenv';
import { checkInit } from './utilsFunctions';
config();

async function uploadProgram() {
	const gearApi = await GearApi.create();
	const jsonKeyring = readFileSync(process.env.PATH_TO_KEYS).toString();
	const account = GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
	const code = readFileSync(process.env.OPT_WASM);
	const metaFile = readFileSync(process.env.META_WASM);
	const meta = await getWasmMetadata(metaFile);
	let initLoan = {
		amount: 10000,
		interest: 10,
		lender: process.env.LENDER,
		borrower: process.env.BORROWER,
		duration: 1000000,
	};

  const addressRaw = GearKeyring.decodeAddress(account.address);

	const gas = await gearApi.program.gasSpent.init(
		addressRaw,
		code,
		initLoan,
		0,
		meta,
	);

	console.log('GAS SPENT', gas.toNumber());

	const { programId } = gearApi.program.submit({ code, initPayload: initLoan, gasLimit: gas}, meta);

  console.log("programId", programId);
  
  const status = checkInit(gearApi, programId);

	await gearApi.program.signAndSend(account, ({events = []}) => {
  	events.forEach(({ event: { method, data }}) => {
			if (method === 'ExtrinsicFailed') {
				throw (data.toHuman());
			}
		});
	});

  console.log(await status);  
}


uploadProgram()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });