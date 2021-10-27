const { test } = require("uvu");
const assert = require("uvu/assert");

const { getFreeSpace } = require("..");

test("returns BigInt", () => {
  const freeSpace = getFreeSpace('.');
  assert.type(freeSpace, "bigint");
});
test("returns the free space in bytes", () => {
  const freeSpace = getFreeSpace('.');
  assert.ok(freeSpace > 0n);
});

test.run();
