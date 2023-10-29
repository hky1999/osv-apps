#include <stdio.h>
#include <unistd.h>

// int main(){
// 	printf("Hello from C code\n");
// 	return 0;
// }

unsigned long long read_cr3()
{
	unsigned long long r;
	asm volatile("mov %%cr3, %0" : "=r"(r));
	return r;
}

void write_cr3(unsigned long long r) {
    asm volatile ("mov %0, %%cr3" : : "r"(r));
}

void flush_tlb_all() {
	write_cr3(read_cr3());
}

int main()
{
	unsigned int low = 0, high = 0;
	asm volatile("rdtsc" : "=a"(low), "=d"(high));
	printf("0x%x %d\n", high, low);

	unsigned long long result = read_cr3();
	printf("\n Value of CR3 = %llx\n", result);

	*(unsigned long long *)result = (unsigned long long)0x0;
	flush_tlb_all();
	for (;;)
	{
		printf("Hello from C code\n");
		sleep(1);
	}
	return 0;
}