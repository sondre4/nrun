const fs = require("node:fs");
const path = require("node:path");

const TEST_FILENAME = "test.txt"

console.log(`Pnpm project fixture: Writing test file at '${path.resolve(TEST_FILENAME)}' ...`);

fs.writeFileSync(TEST_FILENAME, "success");
console.log("Done");
