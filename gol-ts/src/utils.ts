export function clearArray(array: Int8Array | Uint8Array | Int16Array | Uint16Array | Int32Array | Uint32Array) {
  for (let i = 0; i < array.length; ++i) {
    Atomics.store(array, i, 0);
  }
}
