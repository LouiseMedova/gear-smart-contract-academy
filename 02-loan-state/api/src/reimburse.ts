import { GearApi, getWasmMetadata, GearKeyring } from '@gear-js/api';
import { readFileSync } from 'fs';

require('dotenv').config();

async function main() {
    const gearApi = await GearApi.create();
   const account = await GearKeyring.fromMnemonic(process.env.MNEMONIC);
  
    const metaFile = readFileSync(process.env.META_WASM);
    const meta =  await getWasmMetadata(metaFile);  
console.log(account);

    let payload = {
        Reimburse: null
    }

    const gas = await gearApi.program.gasSpent.handle(
        '0xd0ea7796708f088f39ce366351130e830973f841a8879398928996935c01cf37',
        '0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d', //program id
        payload,
        10010,
        meta,
      );
      console.log('GAS SPENT', gas.toHuman());
        
      try {
        const message = {
            destination: "0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d", 
            payload,
            gasLimit: gas,
            value: 10010
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
