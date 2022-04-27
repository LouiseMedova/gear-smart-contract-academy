import { GearApi, getWasmMetadata } from '@gear-js/api';
import { readFileSync } from 'fs';

export const uploadProgram = async (
  api: GearApi, 
  pathToProgram: string, 
  pathToMeta?: string,
  account?: any, 
  value?: any, 
  initPayload?: any) => {
  const code = readFileSync(pathToProgram);
  const metaFile = pathToMeta ? readFileSync(pathToMeta) : undefined;
  const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;
console.log(pathToMeta);

  const gas = await api.program.gasSpent.init(
    account.address,
    code,
    initPayload,
    value, 
    meta
  );

  console.log("GAS SPENT", gas.toHuman());

  const programId = api.program.submit({ code, initPayload, gasLimit: gas }, meta);
  await api.program.signAndSend(account, (data) => {
    console.log(data.toHuman());
  });
  return programId;
  
};