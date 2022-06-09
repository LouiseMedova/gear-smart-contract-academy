import { GearApi, GearKeyring, getWasmMetadata } from '@gear-js/api';
import { readFileSync } from 'fs';

require('dotenv').config();

// function
// check init https://github.com/gear-tech/gear-js/blob/master/api/test/utilsFunctions.ts#L3-L46

async function main() {
  const gearApi = await GearApi.create();
  const jsonKeyring = readFileSync(process.env.PATH_TO_KEYS).toString();
  const account = GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
  const code = readFileSync(process.env.OPT_WASM);
  const metaFile = readFileSync(process.env.META_WASM);
  const meta = await getWasmMetadata(metaFile);
  console.log(process.env.LENDER);
  console.log(process.env.BORROWER);

  let initLoan = {
    amount: 10000,
    interest: 10,
    lender: process.env.LENDER,
    borrower: process.env.BORROWER,
    duration: 1000000,
  };

  const gas = await gearApi.program.gasSpent.init(
    '0x8260b9aae93a8486064217041d5ee6b81a9f716ba428ce20692061a7b3b35662',
    code,
    initLoan,
    0,
    meta,
  );
  console.log('GAS SPENT', gas.toNumber());

  // upload program https://github.com/gear-tech/gear-js/blob/master/api/test/utilsFunctions.ts#L3-L46
  const { programId } = gearApi.program.submit({ code, initPayload: initLoan, gasLimit: gas }, meta);
  await gearApi.program.signAndSend(account, (events) => {
    // Add check
    // Log only MessageEnqueued
    // if event is ExtrinsicFailed than throw error
    events.forEach(({ method, data }) => {
      if (method === 'MessageEnqueud') {
        console.log(data.toHuman());
      }
    });
  });
  // Remove
  console.log('Program was initialized with id', programId);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });
