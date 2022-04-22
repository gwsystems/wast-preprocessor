extern int wasmf_add(int, int);

int main(int argc, char* argv[]) {
	awsm_assert(wasmf_add(1, 1) == 2);
}
