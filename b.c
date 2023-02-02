#include <stdio.h>
#include <string.h>
  
 union Data {
    int i;
    float f;
    char str[20];
 };
  
 int main( ) {

    union Data data;
    int i;
    float f;
    char str[20];

    printf( "int: %ld, float: %ld, char[20]: %ld, total : %ld\n",sizeof(i), sizeof(f), sizeof(str), sizeof(data));

    return 0;
 }

