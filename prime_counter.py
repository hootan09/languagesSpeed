def isPrime(n):
    for i in range(2,n//2+1):
        if(not (n%i)):
            return 0
    return 1

numPrimes=0
for i in range(2,250001):
    numPrimes+=isPrime(i)
print(str(numPrimes))

'''
$ time python ./prime_counter.py
22044

real    1m49.135s
user    0m0.047s
sys     0m0.015s
'''