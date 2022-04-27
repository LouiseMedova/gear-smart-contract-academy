import { GearApi, GearKeyring, getWasmMetadata } from '@gear-js/api';
import { events } from '../../../gear-js-utils/event-listener';

require('dotenv').config();

async function main() {
   
    await events();
}

main()