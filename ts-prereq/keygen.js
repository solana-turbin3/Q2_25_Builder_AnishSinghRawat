"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var web3_js_1 = require("@solana/web3.js");
var kp = web3_js_1.Keypair.generate();
console.log("You've generated a new Solana wallet: ".concat(kp.publicKey.toBase58()));
console.log("[".concat(kp.secretKey, "]"));
