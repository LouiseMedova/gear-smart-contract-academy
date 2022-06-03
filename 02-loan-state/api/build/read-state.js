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
        const metaWasm = (0, fs_1.readFileSync)(process.env.META_WASM);
        const current_state = yield gearApi.programState.read(process.env.PROGRAM_ID, metaWasm, { CurrentState: null });
        console.log(current_state.toHuman());
        const details = yield gearApi.programState.read(process.env.PROGRAM_ID, metaWasm, { Details: null });
        console.log(details.toHuman());
    });
}
;
main();
//# sourceMappingURL=read-state.js.map