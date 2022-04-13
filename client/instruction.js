const borsh = require("borsh");

const CREATE_PROJECT = 0x0;
const COMPILMENT_PROJECT = 0x1;

class createParams {
    target = null;
    name = null;

    constructor(fields) {
        this.target = fields.target;
        this.name = new TextEncoder('utf8').encode(fields.name);

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
        let name = borsh.serialize(getSchema(['u8']), new Value(this.name)).toJSON().data;


        console.log("\nTarget is", target);
        console.log("\nName is", name);
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
                ['name', ['u8']]
            ]
        }
    ],
]);

exports.createParams = createParams;
exports.CREATE_PROJECT = CREATE_PROJECT;
exports.COMPILMENT_PROJECT = COMPILMENT_PROJECT;