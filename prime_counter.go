package main

import ("fmt")

func isPrime(num int) int {
	for i := 2; i<=(num/2); i++ {
      if(num%i == 0){
         return 0;
     }
   }
   return 1;
}

func main() {

   numPrimes := 0;
	for i := 2; i <250001; i++ {
      numPrimes +=isPrime(i);
   }
   fmt.Printf("%d\n", numPrimes)  
}

/*
$ time ./prime_counter.exe
22044

real    0m15.802s
user    0m0.000s
sys     0m0.046s
*/