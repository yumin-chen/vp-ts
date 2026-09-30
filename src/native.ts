import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Import native module built by napi-rs in dist or root
let nativeBinding: any;
try {
  nativeBinding = require("../dist/index.js");
} catch {
  nativeBinding = require("../index.js");
}

export const { customAlphabetNative, formatNative, nanoidNative, URL_ALPHABET } = nativeBinding;
