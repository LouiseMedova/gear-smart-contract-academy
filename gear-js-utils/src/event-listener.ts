import { GearApi } from '@gear-js/api';

export const events = async () => {
    const gearApi = await GearApi.create({ providerAddress: 'wss://node-workshop.gear.rs:443' });

  gearApi.gearEvents.subscribeToLogEvents(({ data: { id, source, payload, reply } }) => {
    console.log(`
      Log:
      messageId: ${id.toHex()}
      from program: ${source.toHex()}
    payload: ${payload.toHuman()}
    ${
      reply.isSome
        ? `reply to: ${reply.unwrap()[0].toHex()}
      with error: ${reply.unwrap()[1].toNumber() === 0 ? false : true}
      `
        : ''
    }
    `);
  });

  gearApi.gearEvents.subscribeToProgramEvents(({ method, data: { info, reason } }) => {
    console.log(`
      ${method}:
      programId: ${info.programId.toHex()}
      initMessageId: ${info.messageId.toHex()}
      origin: ${info.origin.toHex()}
      ${reason ? `reason: ${reason.toHuman()}` : ''}
      `);
  });

//   gearApi.gearEvents.subscribeToTransferEvents(({ data: { from, to, value } }) => {
//     console.log(`
//     Transfer balance:
//     from: ${from.toHex()}
//     to: ${to.toHex()}
//     value: ${+value.toString()}
//     `);
//   });
};