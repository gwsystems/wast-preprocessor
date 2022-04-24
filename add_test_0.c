#include <stdint.h>
#include <assert.h>
extern int wasmf_add(int, int);

int main(int argc, char* argv[]) {
	assert(wasmf_add(1, 1) == 2);
}
