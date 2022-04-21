extern float wasmf_32_good1(int);
extern float wasmf_32_good2(int);
extern float wasmf_32_good3(int);
extern float wasmf_32_good4(int);
extern float wasmf_32_good5(int);
extern void wasmf_32_bad(int);

int main(int argc, char* argv[]) {
	awsm_assert(wasmf_32_good1(0) == (double)0);
	awsm_assert(wasmf_32_good2(0) == (double)0);
	awsm_assert(wasmf_32_good3(0) == (double)0);
	awsm_assert(wasmf_32_good4(0) == (double)0);
	awsm_assert(wasmf_32_good5(0) == (double)2144337921);
	awsm_assert(wasmf_32_good1(65524) == (double)0);
	awsm_assert(wasmf_32_good2(65524) == (double)0);
	awsm_assert(wasmf_32_good3(65524) == (double)0);
	awsm_assert(wasmf_32_good4(65524) == (double)0);
	awsm_assert(wasmf_32_good5(65524) == (double)0);
	awsm_assert(wasmf_32_good1(65525) == (double)0);
	awsm_assert(wasmf_32_good2(65525) == (double)0);
	awsm_assert(wasmf_32_good3(65525) == (double)0);
	awsm_assert(wasmf_32_good4(65525) == (double)0);
}
