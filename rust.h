/*
 * Prototypes for functions migrated to Rust, and definitions for
 * types shared between Rust and C.
 */

#ifndef LibLambdaMoo_h
#define LibLambdaMoo_h 1

#include <stddef.h>
#include <stdint.h>

/* ascii_string.rs */
extern int old_mystrcasecmp(const char *, const char *);
extern int old_mystrncasecmp(const char *, const char *, int);
extern int old_strindex(const char *source, const char *what, int case_counts);
extern int old_strrindex(const char *source, const char *what, int case_counts);
extern char *old_strsub(const char *source, const char *what, const char *with, int case_counts);

/* crypto.rs */
const char * old_hash_bytes(const char *input, int length);

/* memory.rs */
typedef enum Memory_Type {
    M_AST_POOL, M_AST, M_PROGRAM, M_PVAL, M_NETWORK, M_STRING, M_VERBDEF,
    M_LIST, M_PREP, M_PROPDEF, M_OBJECT_TABLE, M_OBJECT, M_FLOAT,
    M_STREAM, M_NAMES, M_ENV, M_TASK, M_PATTERN,

    M_BYTECODES, M_FORK_VECTORS, M_LIT_LIST,
    M_PROTOTYPE, M_CODE_GEN, M_DISASSEMBLE, M_DECOMPILE,

    M_RT_STACK, M_RT_ENV, M_BI_FUNC_DATA, M_VM,

    M_REF_ENTRY, M_REF_TABLE, M_VC_ENTRY, M_VC_TABLE, M_STRING_PTRS,
    M_INTERN_POINTER, M_INTERN_ENTRY, M_INTERN_HUNK,

    Sizeof_Memory_Type

} Memory_Type;

extern char *str_dup(const char *);
extern void myfree(void *where, Memory_Type type);
extern void *almost_mymalloc(size_t size, Memory_Type type);
extern void *almost_myrealloc(void *where, size_t size, Memory_Type type);
extern int32_t addref(void *ptr);
extern int32_t delref(void *ptr);
extern int32_t refcount(void *ptr);

/* parse_cmd.rs */
extern char **old_parse_into_words(char *input, int *nwords);

#endif
