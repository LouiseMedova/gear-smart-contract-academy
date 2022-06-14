import { GearApi, getWasmMetadata, GearKeyring, Hex, UserMessageSent, CreateType } from '@gear-js/api';
import { readFileSync } from 'fs';
import {config } from 'dotenv';
import { checkTransaction } from './utilsFunctions';
config();

async function fund() {
  const gearApi = await GearApi.create();
  const jsonKeyring = readFileSync(process.env.PATH_TO_KEYS).toString();
  const account = GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
  const metaFile = readFileSync(process.env.META_WASM);
  const meta = await getWasmMetadata(metaFile);

  const programId = process.argv.slice(2)[0];
  const addressRaw = GearKeyring.decodeAddress(account.address);

  let payload = {
    Fund: null,
  };

  const gas = await gearApi.program.gasSpent.handle(
    addressRaw,
    programId as Hex,
    payload,
    10000,
    meta,
  );
  console.log('GAS SPENT', gas.toHuman());

  try {
    const message = {
      destination: programId as Hex, 
      payload,
      gasLimit: gas,
      value: 10000,
    };
    await gearApi.message.submit(message, meta);
  } catch (error) {
    console.error(`${error.name}: ${error.message}`);
  }

  const status = checkTransaction(gearApi, programId);

  await gearApi.message.signAndSend(account, ({events = []}) => {
    
    events.forEach(({ event: { method, data }}) => {
      if (method === 'ExtrinsicFailed') {
        throw (data.toHuman());
      }
    });
}) 

  console.log(await status);
  
}

fund()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });
