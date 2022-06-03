import { GearApi, getWasmMetadata, GearKeyring } from '@gear-js/api';
import { readFileSync } from 'fs';

require('dotenv').config();

async function main() {
    const gearApi = await GearApi.create();
    const jsonKeyring = readFileSync(process.env.PATH_TO_KEYS).toString();
    const account = GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
    const metaFile = readFileSync(process.env.META_WASM);
    const meta =  await getWasmMetadata(metaFile);  

    let payload = {
        Fund: null
    }

    const gas = await gearApi.program.gasSpent.handle(
        '0x8260b9aae93a8486064217041d5ee6b81a9f716ba428ce20692061a7b3b35662',
        '0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d', //program id
        payload,
        10000,
        meta,
      );
      console.log('GAS SPENT', gas.toHuman());
        console.log('program id', process.env.PROGRAM_ID);
        
      try {
        const message = {
            destination: "0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d", 
            payload,
            gasLimit: gas,
            value: 10000
        };
        await gearApi.message.submit(message, meta);        
      } catch (error) {
        console.error(`${error.name}: ${error.message}`);
      }
      try {
        await gearApi.message.signAndSend(account, (event) => {
          console.log(event.toHuman());
        });
      } catch (error) {
        console.error(`${error.name}: ${error.message}`);
      }
    
}
 
 main()
