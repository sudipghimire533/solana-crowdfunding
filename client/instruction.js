const borsh = require("borsh");

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
        return borsh.serialize(ComplimentParamsSchema, this);
    }
}

const ComplimentParamsSchema = new Map([
    [complimentParams,
        {
            kind: 'struct',
            fields: [
                ['amount', 'u64']
            ]
        }
    ]
]);

exports.createParams = createParams;
exports.complimentParams = complimentParams;
exports.CREATE_PROJECT = CREATE_PROJECT;
exports.COMPILMENT_PROJECT = COMPILMENT_PROJECT;