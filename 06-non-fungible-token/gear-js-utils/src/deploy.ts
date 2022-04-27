import { GearApi, GearKeyring, getWasmMetadata } from '@gear-js/api';
import { uploadProgram } from './upload-program';

require('dotenv').config();

async function main() {
  const gearApi = await GearApi.create({ providerAddress: 'wss://node-workshop.gear.rs:443' });
  const account = await GearKeyring.fromMnemonic("honey bless drink pumpkin basic swarm slot quick salon medal match ability");
 
  console.log("start deploying non-fungible-token");
  console.log(account.address);
  
  const initNFT = {
    name: "My Token",
    symbol: "MTK",
    base_uri: "http:"
  }
  
  const token_program_id = await uploadProgram(
    gearApi,
    process.env.NFT_OPT || "",
    process.env.NFT_META,
    account,
    initNFT
  )

  console.log("Non-Fungible-Token Program ID:", token_program_id.programId);

}

main()