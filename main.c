#include <stdio.h>
#include <stdlib.h>

// print int value by binary 0 and 1
#define INTBIT_SIZE 32
void print_binary(int value, char *key) {
  int tmp = value;
  char *ptr = (char *)malloc(sizeof(INTBIT_SIZE + 1));
  if (!ptr)
    return;
  ptr[INTBIT_SIZE] = '\0';
  for (unsigned long i = 0; i < INTBIT_SIZE; i++) {
    ptr[INTBIT_SIZE - i - 1] = tmp & 1 ? '1' : '0';
    tmp >>= 1;
  }
  printf("%s result : %s\n", key, ptr);
  free(ptr);
}

int main(void) {
  int value = 32982;

  print_binary(value, "value");
  return 0;
}