'use strict';

const nativeQuery = require('./native-query');
const readRows = require('./read-rows');
const render = require('./render');
const crossCheckQuery = require('./cross-check-query');
const errorLine = require('./error-line');

const DISAGREEMENT = 'fault: public query disagrees';

// A SELECT inside an open transaction: it sees the not-yet-committed working
// state and never releases the instance (the suite keeps running on it).
function queryInstruction(instance, sql) {
  try {
    const { columns, rows } = readRows(nativeQuery(instance.handle, sql));
    if (!crossCheckQuery(instance, sql, columns, rows)) return DISAGREEMENT;
    return render(columns, rows).replace(/\n+$/, '');
  } catch (error) {
    return errorLine(error);
  }
}

module.exports = queryInstruction;
