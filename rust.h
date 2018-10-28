/*
 * Prototypes for functions migrated to Rust, and definitions for
 * types shared between Rust and C.
 */

#ifndef LibLambdaMoo_h
#define LibLambdaMoo_h 1

#include <stddef.h>
#include <stdint.h>

/* crypto.rs */
const char * old_hash_bytes(const char *input, int length);

/* memory.rs */
extern char *str_dup(const char *);
extern void myfree(void *where, uint32_t type);
extern void *almost_mymalloc(size_t size, uint32_t type);
extern void *almost_myrealloc(void *where, size_t size, uint32_t type);

/* parse_cmd.rs */
extern char **old_parse_into_words(char *input, int *nwords);

#endif
