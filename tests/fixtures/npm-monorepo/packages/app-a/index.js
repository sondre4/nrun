const fs = require("node:fs");
const path = require("node:path");

const TEST_FILENAME = "test.txt"

console.log(`Npm monorepo fixture package 'app-a': Writing test file at '${path.resolve(TEST_FILENAME)}' ...`);

fs.writeFileSync(TEST_FILENAME, "success");
console.log("Done");
