const { bindings } = require("@wasmer/sha2-wasm");
const { Hasher } = require("@wasmer/sha2-wasm/src/bindings/sha2/sha2.js");

function buf2hex(buffer) {
  return [...new Uint8Array(buffer)]
    .map((x) => x.toString(16).padStart(2, "0"))
    .join("");
}

const uInt8ArrFromString = (str) => new Uint8Array(Buffer.from(str, "utf-8"));

test("Test Sha256 Compress", async () => {
  const wasm = await bindings.sha2();
  const testStringByteBuffer = new Uint8Array(
    Buffer.from("hello world", "utf-8")
  );
  const resultBuffer = wasm.sha256(testStringByteBuffer).buffer;
  expect(buf2hex(resultBuffer)).toBe(
    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
  );
});

test("Test Sha512 Compress", async () => {
  const wasm = await bindings.sha2();
  const testStringByteBuffer = new Uint8Array(
    Buffer.from("hello world", "utf-8")
  );
  const resultBuffer = wasm.sha512(testStringByteBuffer).buffer;
  expect(buf2hex(resultBuffer)).toBe(
    "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
  );
});

test("Test Hasher Sha256", async () => {
  const wasm = await bindings.sha2();
  const hasher = Hasher.sha256(wasm);
  hasher.update(uInt8ArrFromString("hello"));
  hasher.update(uInt8ArrFromString(" "));
  hasher.update(uInt8ArrFromString("world"));

  const resultBuffer = hasher.finalize().buffer;
  expect(buf2hex(resultBuffer)).toBe(
    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
  );
});
test("Test Hasher Sha512", async () => {
  const wasm = await bindings.sha2();
  const hasher = Hasher.sha512(wasm);
  hasher.update(uInt8ArrFromString("hello"));
  hasher.update(uInt8ArrFromString(" "));
  hasher.update(uInt8ArrFromString("world"));

  const resultBuffer = hasher.finalize().buffer;
  expect(buf2hex(resultBuffer)).toBe(
    "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
  );
});
