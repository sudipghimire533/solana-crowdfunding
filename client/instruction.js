const borsh = require("borsh");

const CREATE_PROJECT = 0x0;
const COMPILMENT_PROJECT = 1;

class createParams {
    target = null;
    bank = null;
    owner = null;
    name = null;
    address = null;

    constructor(fields) {
        this.target = fields.target;
        this.bank = fields.bank;
        this.owner = fields.owner;
        this.name = new TextEncoder('utf8').encode(fields.name);
        this.address = fields.address;

        class Value {
            value = null;

            constructor(param) {
                this.value = param;
            }
        }

        function getSchema(type) {
            return new Map([
                [Value, {
                    kind: 'struct',
                    fields: [
                        ['value', type]
                    ]
                }]
            ]);
        }

        let target = borsh.serialize(getSchema('u64'), new Value(this.target)).toJSON().data;
        let bank = borsh.serialize(getSchema(['u8', '32']), new Value(this.bank)).toJSON().data;
        let owner = borsh.serialize(getSchema(['u8', '32']), new Value(this.owner)).toJSON().data;
        let address = borsh.serialize(getSchema(['u8', '32']), new Value(this.address)).toJSON().data;
        let name = borsh.serialize(getSchema(['u8']), new Value(this.name)).toJSON().data;


        console.log("\nTarget is", target);
        console.log("\nBank is", bank);
        console.log("\nOwner is", owner);
        console.log("\nName is", name);
        console.log("\nAddress is", address);
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
                ['bank', ['u8', '32']],
                ['owner', ['u8', '32']],
                ['name', ['u8']],
                ['address', ['u8', '32']]
            ]
        }
    ],
]);

exports.createParams = createParams;
exports.CREATE_PROJECT = CREATE_PROJECT;
exports.COMPILMENT_PROJECT = COMPILMENT_PROJECT;