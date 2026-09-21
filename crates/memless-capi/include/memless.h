#define FFI_SCOPE "memless"

typedef uint64_t MemlessHandle;

/* 0 = Ok, 1 = Refused, 2 = InvalidArgument, 3 = Internal */
typedef int32_t MemlessStatus;

uint32_t memless_abi_version(void);

/*
 * Loads an instance on `path`.
 * When `out_message` is non-null it is always written: NULL on Ok, otherwise an
 * owned message the caller must release with `memless_free_string`. On Ok,
 * `*out_handle` receives a handle >= 1. `out_handle` and `out_message` may be
 * null; a null pointer is never written through.
 */
MemlessStatus memless_load(const char *path, MemlessHandle *out_handle, char **out_message);

/* Releases the instance behind `handle`; an unknown or zero handle is ignored. */
void memless_release(MemlessHandle handle);

/* Frees a message returned through `out_message`; NULL is ignored. */
void memless_free_string(char *message);
