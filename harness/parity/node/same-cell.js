'use strict';

// bigint and number compare by exact numeric equality: an integer beyond the
// safe range is a BigInt on whichever side produced it, never a text mismatch
// between "9007199254740993" and a BigInt of the same value (brief S8.8).
function sameCell(raw, pub) {
  if (typeof raw === 'bigint' || typeof pub === 'bigint') return BigInt(raw) === BigInt(pub);
  return raw === pub;
}

module.exports = sameCell;
