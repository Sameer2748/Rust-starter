import * as borsh from "borsh";


export class CounterAccount {
    count: number;

    constructor({ count }: { count: number }) {
        this.count = count;
    }

}

export const schema: borsh.Schema = { struct: { count: "u32" } };


export const Size = borsh.serialize(
    schema,
    new CounterAccount({count:10})
).length;

//serialize
// console.log(Size);
//deseerialize
// console.log(borsh.deserialize(schema, Size));

export class IncrementInstruction {
    instruction: number; // Should be 0 for the Increment variant
    value: number;
  
    constructor({ value }: { value: number }) {
      this.instruction = 0; // 0 corresponds to the Increment variant in your Rust enum
      this.value = value;
    }
  }
  
  // Define the schema as a plain object (similar to your counter schema)
  export const instructionSchema: borsh.Schema = {
    struct: {
      instruction: "u8",
      value: "u32",
    },
  };

  

  export class DecrementInstruction{
    instruction: number; // Should be 0 for the Increment variant
    value: number;
  
    constructor({ value }: { value: number }) {
      this.instruction = 1; // 0 corresponds to the Increment variant in your Rust enum
      this.value = value;
    }
  }
