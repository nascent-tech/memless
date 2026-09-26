'use strict';

const sameColumns = require('./same-columns');
const sameRow = require('./same-row');

// Runs sql again through the bridge's public Instance#query and compares it
// cell by cell against the raw read (M4): a real divergence between the
// internal read and the public surface must never pass as agreement, and a
// public-side throw where the raw read succeeded is itself a disagreement.
function crossCheckQuery(instance, sql, columns, rows) {
  let publicResult;
  try {
    publicResult = instance.query(sql);
  } catch {
    return false;
  }
  if (!sameColumns(publicResult.columns, columns)) return false;
  if (publicResult.rows.length !== rows.length) return false;
  return publicResult.rows.every((row, index) => sameRow(row, rows[index]));
}

module.exports = crossCheckQuery;
