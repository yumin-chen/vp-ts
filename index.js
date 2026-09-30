const nativeBinding = require('./build/index.js');

const urlAlphabet =
  'usemodule-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ1234567890_-';

function nanoid(size) {
  return nativeBinding.nanoid(size);
}

function customAlphabet(alphabet, defaultSize = 21) {
  if (typeof alphabet !== 'string' || alphabet.length === 0 || alphabet.length > 256) {
    throw new Error('Alphabet must contain from 1 to 256 symbols');
  }
  return (size = defaultSize) => {
    return nativeBinding.customAlphabet(alphabet, size);
  };
}

function customRandom(alphabet, defaultSize, random) {
  if (typeof alphabet !== 'string' || alphabet.length === 0 || alphabet.length > 256) {
    throw new Error('Alphabet must contain from 1 to 256 symbols');
  }
  const mask = (2 << (31 - Math.clz32((alphabet.length - 1) | 1))) - 1;

  return (size = defaultSize) => {
    const step = Math.ceil((1.6 * mask * size) / alphabet.length);
    let id = '';
    while (true) {
      const bytes = random(step);
      let i = step;
      while (i--) {
        id += alphabet[bytes[i] & mask] || '';
        if (id.length === size) return id;
      }
    }
  };
}

module.exports = {
  urlAlphabet,
  nanoid,
  customAlphabet,
  customRandom,
};
