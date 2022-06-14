import { GearApi, CreateType, getWasmMetadata } from '@gear-js/api';
import { readFileSync } from 'fs';
require('dotenv').config();
export const events = async (pathToMeta?: string) => {
  const gearApi = await GearApi.create();

  const metaFile = pathToMeta ? readFileSync(pathToMeta) : undefined;
  const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;
  console.log(meta);

  //function litenToUserMessageSent

  gearApi.gearEvents.subscribeToGearEvent('MessageEnqueued', (event) => {
    console.log(event);
    
    if (event.data.entry.isInit) {
      console.log(event.data.id.toHex());
    }
  })

  //   try {
  //     console.log(CreateType.create(meta.handle_output, payload, meta).toHuman());
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });

}

async function main() {
  await events(process.env.META_WASM);
}

main();
