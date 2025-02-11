#include <stdio.h>

void swap(int a,int b){
    int t = a;
    a = b;
    b = t;
}

int main(){
    int a = 5;
    int b = 10;
    printf("A : %d \n",a);
    printf("B : %d \n",b);
    swap(a,b);
    printf("A : %d \n",a);
    printf("B : %d \n",b);

}