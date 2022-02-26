#include <stdint.h>

void reax_init_runtime();
void reax_init_store();
void reax_init_handler(const void * ptr, void (*callback)( int *subs, int subs_len, const unsigned char *bytes, int bytes_len, const void * ptr));

int reax_user();
void reax_fetch_user();
void reax_unsubscribe(int);
