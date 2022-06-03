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
const api_1 = require("@gear-js/api");
const fs_1 = require("fs");
require('dotenv').config();
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const gearApi = yield api_1.GearApi.create();
        const jsonKeyring = (0, fs_1.readFileSync)(process.env.PATH_TO_KEYS).toString();
        const account = api_1.GearKeyring.fromJson(jsonKeyring, process.env.PASSWORD);
        const metaFile = (0, fs_1.readFileSync)(process.env.META_WASM);
        const meta = yield (0, api_1.getWasmMetadata)(metaFile);
        let payload = {
            Fund: null
        };
        const gas = yield gearApi.program.gasSpent.handle('0x8260b9aae93a8486064217041d5ee6b81a9f716ba428ce20692061a7b3b35662', '0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d', payload, 10000, meta);
        console.log('GAS SPENT', gas.toHuman());
        console.log('program id', process.env.PROGRAM_ID);
        try {
            const message = {
                destination: "0xf14b3356a630872393a3e041980ed246d829046af2da212d75efe2806e07ff3d",
                payload,
                gasLimit: gas,
                value: 10000
            };
            yield gearApi.message.submit(message, meta);
        }
        catch (error) {
            console.error(`${error.name}: ${error.message}`);
        }
        try {
            yield gearApi.message.signAndSend(account, (event) => {
                console.log(event.toHuman());
            });
        }
        catch (error) {
            console.error(`${error.name}: ${error.message}`);
        }
    });
}
main();
//# sourceMappingURL=fund.js.map