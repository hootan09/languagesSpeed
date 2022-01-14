function isPrime(num){
    for(let i=2; i<=(num/2); i++){
        if(!(num%i)){
            return 0;
        }
    }
    return 1;
}

function main (){
    let numPrimes=0;

    for(let i = 2;i<250001; i++){
        numPrimes +=isPrime(i);
    }

    console.log(numPrimes);
}

main();

/**
 $ time node ./prime_counter.js 
22044

real    0m9.892s
user    0m0.046s
sys     0m0.077s
 */