'use strict';

const nativeQuery = require('./native-query');
const readRows = require('./read-rows');
const render = require('./render');
const crossCheckQuery = require('./cross-check-query');
const errorLine = require('./error-line');

const DISAGREEMENT = 'fault: public query disagrees';

// Runs sql on an already-loaded instance and always releases it.
function runQuery(instance, sql) {
  try {
    const { columns, rows } = readRows(nativeQuery(instance.handle, sql));
    if (!crossCheckQuery(instance, sql, columns, rows)) return DISAGREEMENT + '\n';
    return render(columns, rows);
  } catch (error) {
    return errorLine(error) + '\n';
  } finally {
    instance.release();
  }
}

module.exports = runQuery;
