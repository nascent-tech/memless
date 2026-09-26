// Compiled with `tsc --noEmit --strict`, never run: proves index.d.ts matches
// the public surface a TypeScript caller actually sees.
import { load, Instance, MemlessRefusal, MemlessFault, QueryResult, Cell } from '../src/index';

const db: Instance = load('data.yaml');

const handle: number | bigint = db.handle;
const released: boolean = db.released;

const result: QueryResult = db.query('SELECT name FROM users');
const columns: string[] = result.columns;
const rows: Cell[][] = result.rows;
const first: Cell = rows[0][0];

const affected: number = db.execute("DELETE FROM users WHERE id = '01H7B2'");

db.begin();
db.commit();
db.rollback();
db.release();

try {
  load('missing.yaml');
} catch (error) {
  if (error instanceof MemlessRefusal) {
    const message: string = error.message;
  } else if (error instanceof MemlessFault) {
    const status: number = error.status;
  }
}

void columns;
void first;
void affected;
void handle;
void released;
