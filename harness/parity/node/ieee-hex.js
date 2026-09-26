'use strict';

// Renders a JS double as its big-endian IEEE-754 bit pattern in hex, matching
// PHP's pack('E', ...)/unpack('J', ...) and Go's math.Float64bits.
function ieeeHex(value) {
  const buffer = Buffer.alloc(8);
  buffer.writeDoubleBE(value);
  return buffer.toString('hex');
}

module.exports = ieeeHex;
