import { run, bench, group } from 'mitata'
import {
  processSmallUserValidated,
  processSerdeJson,
  DynamicSchema,
  DynamicSchemaType,
  saveToDynamicDbJs
} from './index.cjs'

// Create a large object with 100 fields
const largeObject = {
  id: 1,
  name: 'John Doe',
};

for (let i = 0; i < 100; i++) {
  largeObject[`field${i}`] = `value${i}`;
}

const smallSchema = new DynamicSchema({
  id: DynamicSchemaType.Number,
  name: DynamicSchemaType.String
});

group('Validation Performance (100 fields, accessing 2)', () => {
  bench('serde_json::Value (Full Copy)', () => {
    processSerdeJson(largeObject);
  });

  bench('Validated<SmallUserSchema> (Recursive, Zero-Copy Access)', () => {
    processSmallUserValidated(largeObject);
  });

  bench('DynamicValidated (Runtime Schema, Zero-Copy Access)', () => {
    saveToDynamicDbJs(largeObject, smallSchema);
  });
});

group('Lazy vs Eager Access (100 fields)', () => {
  bench('serde_json::Value (Convert entire object)', () => {
    processSerdeJson(largeObject);
  });

  bench('Validated<T> (Only access name)', () => {
    processSmallUserValidated({ id: 1, name: 'John Doe' }); // small object for baseline
  });
});

await run();
