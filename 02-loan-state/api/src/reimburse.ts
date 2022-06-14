import { GearApi, getWasmMetadata, GearKeyring, Hex } from '@gear-js/api';
import { readFileSync } from 'fs';
import { config } from 'dotenv';
import { checkTransaction } from './utilsFunctions';
config();

async function reimburse() {
  const gearApi = await GearApi.create();
  const account = await GearKeyring.fromMnemonic(process.env.MNEMONIC);

  const metaFile = readFileSync(process.env.META_WASM);
  const meta = await getWasmMetadata(metaFile);
  const programId = process.argv.slice(2)[0];
  const addressRaw = GearKeyring.decodeAddress(account.address);

  let payload = {
    Reimburse: null,
  };


  const gas = await gearApi.program.gasSpent.handle(
    addressRaw,
    programId as Hex, 
    payload,
    10010,
    meta,
  );
  console.log('GAS SPENT', gas.toHuman());

  try {
    const message = {
      destination: programId as Hex,
      payload,
      gasLimit: gas,
      value: 10010,
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

reimburse()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });
