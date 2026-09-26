'use strict';

const { INTEGER, DECIMAL, BOOLEAN } = require('@nascent-tech/memless/src/kind');
const escape = require('./escape');
const ieeeHex = require('./ieee-hex');

// Renders one already-converted {kind, value} cell as the canonical protocol
// tag: null, int:<n> (exact, a BigInt beyond the safe range prints with no
// trailing "n"), dec:<hex> (IEEE-754 bits, computed from the value itself —
// a double's bit pattern is fully determined by its value), bool:true|false,
// or str:<escaped>.
function renderCell(kind, value) {
  if (kind === DECIMAL) return 'dec:' + ieeeHex(value);
  if (value === null) return 'null';
  if (kind === INTEGER) return 'int:' + value.toString();
  if (kind === BOOLEAN) return 'bool:' + (value ? 'true' : 'false');
  return 'str:' + escape(value);
}

module.exports = renderCell;
