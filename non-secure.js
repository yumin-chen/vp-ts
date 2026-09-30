const urlAlphabet = "usemodule-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890_-";

function customAlphabet(alphabet, defaultSize = 21) {
  if (typeof alphabet !== "string" || alphabet.length === 0 || alphabet.length > 256) {
    throw new Error("Alphabet must contain from 1 to 256 symbols");
  }
  return (size = defaultSize) => {
    let id = "";
    let i = size;
    while (i--) {
      id += alphabet[(Math.random() * alphabet.length) | 0];
    }
    return id;
  };
}

function nanoid(size = 21) {
  return customAlphabet(urlAlphabet, size)();
}

module.exports = {
  urlAlphabet,
  nanoid,
  customAlphabet,
};
