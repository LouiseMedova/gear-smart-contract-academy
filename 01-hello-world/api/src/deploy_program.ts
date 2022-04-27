import { GearApi, GearKeyring} from '@gear-js/api';
import { uploadProgram } from 'gear-js-utils';

import { readFileSync } from 'fs';

require('dotenv').config();
require('gear-js-utils');

async function main() {
  const gearApi = await GearApi.create();
 // const account = await GearKeyring.fromMnemonic("honey bless drink pumpkin basic swarm slot quick salon medal match ability");
 const jsonKeyring = readFileSync('/Users/louise/smart-contract-academy/01-hello-world/api/keys.json').toString();
 const account = GearKeyring.fromJson(jsonKeyring, '123456');
  console.log("start deploying program");
  
  let program = await uploadProgram(
    gearApi,
    process.env.OPT_WASM || "",
    process.env.META_WASM || "",
    account,
    0,
    0x00
  )
  console.log("Hello Program ID:", program.programId);

}

main()