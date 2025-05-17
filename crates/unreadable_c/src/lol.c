#include <stdio.h>
#include <stdlib.h>

void *standard_compliant_malloc(size_t size) {
	printf("Not allocating %zu bytes\n", size);
	return NULL;
}

void call_unreadable(int ****(*(**(*p)(int *[], void *(*)(size_t)))(int, int, int ***))(int, int)) {
	int arr[] = {123, 456, 789};
	int *arrp = arr;
	int ****(*(**func1)(int, int, int ***))(int, int) = p((&arrp), standard_compliant_malloc);
	if (func1 == NULL) {
		printf("func1 is NULL\n");
		return;
	}
	int ****(*func2)(int, int) = (*func1)(1, 2, NULL);
	if (func2 == NULL) {
		printf("func2 is NULL\n");
		return;
	}
	int ****finally = func2(1, 2);
	printf("finally: %p\n", finally);
}
