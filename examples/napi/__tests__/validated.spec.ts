import test from 'ava'

import {
  acceptValidatedObject,
  createValidatedObject,
  acceptDeepValidatedObject,
  saveUserToCustomDb,
  validateAndProcessDynamic
} from '../index.cjs'

test('should be able to accept validated object', (t) => {
  const obj = {
    name: 'hello',
    nested: {
      count: 42,
    },
    optional: 1,
  }
  t.is(acceptValidatedObject(obj), 42 + 1 + 5)
})

test('should fail if required property is missing', (t) => {
  const obj = {
    name: 'hello',
    nested: {},
  }
  t.throws(() => acceptValidatedObject(obj as any), {
    message: /Object property 'nested' type mismatch. Expect value to be Number, but received Undefined/
  })
})

test('should perform deep validation', (t) => {
  const obj = {
    name: 'hello',
    nested: {
      count: 'invalid',
    },
  }
  t.throws(() => acceptValidatedObject(obj as any))
})

test('should perform deep validation on collections', (t) => {
  t.is(acceptDeepValidatedObject({
    list: [1, 2, 3],
    map: {
      a: { count: 1 }
    }
  }), 4)

  t.throws(() => acceptDeepValidatedObject({
    list: [1, 'invalid', 3],
    map: {
      a: { count: 1 }
    }
  } as any))

  t.throws(() => acceptDeepValidatedObject({
    list: [1, 2, 3],
    map: {
      a: { count: 'invalid' }
    }
  } as any))
})

test('should be able to create validated object', (t) => {
  const obj = createValidatedObject()
  t.is(obj.name, 'hello')
  t.is(obj.nested.count, 42)
})

test('CustomDB save user example', (t) => {
  const user = {
    id: 123,
    profile: {
      username: 'jules',
      email: 'jules@example.com'
    },
    tags: ['rust', 'napi']
  }
  t.is(saveUserToCustomDb(user), 'Saved user jules with ID 123')
})

test('dynamic validation parse example', (t) => {
  const user = {
    id: 456,
    profile: {
      username: 'dynamic',
      email: 'dynamic@example.com'
    },
    tags: []
  }
  t.is(validateAndProcessDynamic(user), 'Dynamic validation successful for user ID 456')

  t.throws(() => validateAndProcessDynamic({ id: 'not a number' } as any))
})
