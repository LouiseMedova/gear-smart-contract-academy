"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.events = void 0;
const api_1 = require("@gear-js/api");
const fs_1 = require("fs");
require('dotenv').config();
const events = (pathToMeta) => __awaiter(void 0, void 0, void 0, function* () {
    const gearApi = yield api_1.GearApi.create();
    const metaFile = pathToMeta ? (0, fs_1.readFileSync)(pathToMeta) : undefined;
    const meta = metaFile ? yield (0, api_1.getWasmMetadata)(metaFile) : undefined;
    console.log(meta);
    gearApi.gearEvents.subscribeToLogEvents(({ data: { id, source, payload, reply } }) => {
        console.log(`
          Log:
          messageId: ${id.toHex()}
          from program: ${source.toHex()}
        payload: ${payload.toHuman()}
        ${reply.isSome
            ? `reply to: ${reply.unwrap()[0].toHex()}
          with error: ${reply.unwrap()[1].toNumber() === 0 ? false : true}
          `
            : ''}
        `);
        try {
            console.log(api_1.CreateType.create(meta.handle_output, payload, meta).toHuman());
        }
        catch (error) {
            console.log(error);
        }
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
});
exports.events = events;
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        yield (0, exports.events)(process.env.META_WASM);
    });
}
main();
//# sourceMappingURL=subscribe.js.map