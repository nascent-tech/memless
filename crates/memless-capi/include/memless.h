#define FFI_SCOPE "memless"

typedef uint64_t MemlessHandle;
typedef uint64_t MemlessResult;

/* 0 = Ok, 1 = Refused, 2 = InvalidArgument, 3 = Internal */
typedef int32_t MemlessStatus;

/* 0 = Absent, 1 = Text, 2 = Integer, 3 = Decimal, 4 = Boolean */
typedef int32_t MemlessKind;

/* ABI version 2. */
uint32_t memless_abi_version(void);

/*
 * Loads an instance on `path`.
 * When `out_message` is non-null it is always written: NULL on Ok, otherwise an
 * owned message the caller must release with `memless_free_string`. On Ok,
 * `*out_handle` receives a handle >= 1. `out_handle` and `out_message` may be
 * null; a null pointer is never written through.
 */
MemlessStatus memless_load(const char *path, MemlessHandle *out_handle, char **out_message);

/*
 * Runs `sql` against the instance behind `handle`.
 * When `out_message` is non-null it is always written: NULL on Ok, otherwise an
 * owned message the caller must release with `memless_free_string`. On Ok,
 * `*out_result` receives a result handle >= 1 the caller must release with
 * `memless_result_release`. A null `sql` or `out_result`, or an unknown handle,
 * yields InvalidArgument. `out_result` and `out_message` may be null; a null
 * pointer is never written through.
 *
 * The result is an independent copy: it stays valid after the instance is
 * released. Borrowed pointers (see below) are NOT synchronised with a
 * concurrent memless_result_release of the same result; the caller must not
 * release a result while it still holds a pointer into it.
 */
MemlessStatus memless_query(MemlessHandle handle, const char *sql, MemlessResult *out_result, char **out_message);

/* Number of columns / rows in the result; 0 for an unknown result. */
uint64_t memless_result_column_count(MemlessResult result);
uint64_t memless_result_row_count(MemlessResult result);

/* Column name at `index`, borrowed until memless_result_release; NULL out of range. */
const char *memless_result_column(MemlessResult result, uint64_t index);

/*
 * Reads the cell at (`row`, `column`). Returns its kind and, for a present
 * scalar, writes one payload: Text sets `*out_text` to a borrowed pointer valid
 * until memless_result_release; Integer/Decimal/Boolean set their out_* value.
 * Any out_* pointer may be null. Absent (also out of range) writes nothing.
 */
MemlessKind memless_result_cell(MemlessResult result, uint64_t row, uint64_t column,
                                int64_t *out_integer, double *out_decimal,
                                int32_t *out_boolean, const char **out_text);

/* Releases the result behind `result`; an unknown or zero result is ignored. */
void memless_result_release(MemlessResult result);

/* Releases the instance behind `handle`; an unknown or zero handle is ignored. */
void memless_release(MemlessHandle handle);

/* Frees a message returned through `out_message`; NULL is ignored. */
void memless_free_string(char *message);
