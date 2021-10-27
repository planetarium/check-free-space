
const { test } = require("uvu");
const assert = require("uvu/assert");

const { checkFreeSpace } = require("..");

test("accepts number", () => {
	assert.is(checkFreeSpace('.', 1024), true);
})

test("accepts string", () => {
	assert.is(checkFreeSpace('.', '1024'), true);
})

test("accepts bigint", () => {
	assert.is(checkFreeSpace('.', 1024n), true);
})