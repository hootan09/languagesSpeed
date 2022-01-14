#include <stdio.h>

int isPrime(int n){
    for(int i=2;i<=n/2;i++)
        if(!(n%i))
            return 0;
    return 1;
}

void main(){
    int numPrimes=0;

    for(int i=2;i<250001;i++)
        numPrimes+=isPrime(i);
    
    printf("%d\n", numPrimes);
}

/*
$ time ./prime_counter.exe
22044

real    0m6.648s
user    0m6.608s
sys     0m0.015s
*/