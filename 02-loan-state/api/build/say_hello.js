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
exports.sendMessage = void 0;
const api_1 = require("@gear-js/api");
const fs_1 = require("fs");
require('dotenv').config();
const sendMessage = (api, destination, pathToMeta, account, value, payload) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const metaFile = pathToMeta ? (0, fs_1.readFileSync)(pathToMeta) : undefined;
        const meta = metaFile ? yield (0, api_1.getWasmMetadata)(metaFile) : undefined;
        const gas = yield api.program.gasSpent.handle(account.address, destination, payload, value, meta);
        console.log('GAS SPENT', gas.toHuman());
        const message = {
            destination: destination,
            payload,
            gasLimit: gas,
            value
        };
        yield api.message.submit(message, meta);
    }
    catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
    try {
        yield api.message.signAndSend(account, (data) => {
            console.log(data.toHuman());
        });
    }
    catch (error) {
        console.error(`${error.name}: ${error.message}`);
    }
});
exports.sendMessage = sendMessage;
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const gearApi = yield api_1.GearApi.create();
        const jsonKeyring = (0, fs_1.readFileSync)('/Users/louise/smart-contract-academy/01-hello-world/api/keys.json').toString();
        const account = api_1.GearKeyring.fromJson(jsonKeyring, '123456');
        const payload = "Hello";
        console.log(process.env.PROGRAM_ID);
        console.log(process.env.META_WASM);
        yield (0, exports.sendMessage)(gearApi, process.env.PROGRAM_ID, process.env.META_WASM, account, 0, payload);
    });
}
main();
//# sourceMappingURL=say_hello.js.map