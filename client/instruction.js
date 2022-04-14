const borsh = require("borsh");
const solana = require("@solana/web3.js");
const buf = require("buffer");

const CREATE_PROJECT = 0x0;
const COMPILMENT_PROJECT = 0x1;

class createParams {
    target = null;
    name = null;
    project_bump = null;

    constructor(fields) {
        this.target = fields.target;
        this.name = new TextEncoder('utf8').encode(fields.name);
        this.project_bump = fields.project_bump;
    }

    serialize = function () {
        return borsh.serialize(createParamsSchema, this);
    }
}

const createParamsSchema = new Map([
    [createParams,
        {
            kind: 'struct',
            fields: [
                ['target', 'u64'],
                ['name', ['u8']],
                ['project_bump', 'u8'],
            ]
        }
    ],
]);

class complimentParams {
    amount = null;

    constructor(amount) {
        this.amount = amount;
    }

    serialize = function () {
        return borsh.serialize(complimentParamsSchema, this);
    }
}

const complimentParamsSchema = new Map([
    [complimentParams,
        {
            kind: 'struct',
            fields: [
                ['amount', 'u64']
            ]
        }
    ]
]);

class projectInfo {
    bank = null;
    owner = null;
    milestone = null;
    raised = null;
    name = null;

    deserialize = function (bytes) {
        let bank_address_bytes = bytes.slice(0, 32);
        let rest = bytes.slice(32)
        let bank_address = new solana.PublicKey(bank_address_bytes);

        let owner_address_bytes = rest.slice(0, 32);
        rest = rest.slice(32);
        let owner_address = new solana.PublicKey(owner_address_bytes);

        // TODO:
        // deserialize other fields too

        this.bank = bank_address;
        this.owner = owner_address;
    }
}

const projectInfoSchema = new Map([
    [projectInfo,
        {
            kind: 'struct',
            fields: [
                ['bank', ['u8', '32']],
                ['owner', ['u8', '32']],
                ['milestone', 'u64'],
                ['raised', 'u64'],
                ['name', ['u8']],
            ]
        }
    ]
]);

exports.projectInfo = projectInfo;
exports.createParams = createParams;
exports.complimentParams = complimentParams;
exports.CREATE_PROJECT = CREATE_PROJECT;
exports.COMPILMENT_PROJECT = COMPILMENT_PROJECT;