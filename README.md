# nlsd-js

A [Natural Language Structured Document](https://github.com/RiddleAndCode/nlsd/tree/master/nlsd) serializer and deserializer JavaScript / TypeScript wrappers. This library uses Rust to WASM under the hood.

## Usage

```javascript
(async () => {
    const nlsd = await import('nlsd'); // webpack requires async import for WASM modules

    const st = nlsd.to_string([1, "hello"]);
    console.log(st); // the list where an item is 1 and another item is `hello`

    const obj = nlsd.from_string(st);
    console.log(obj); // [1, "hello"]
})();

```
