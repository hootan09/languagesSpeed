fn is_prime(n: i32) -> bool
{
    for i in 2..n
    {
        let p = n % i;
        if p == 0
        {
            return false;
        }
}
    return true;
}
fn main()
{
    loop
    {
        let r: i32 = 250001;

        let mut y: i32 = 0;
        for x in 2..=r
        {
            if is_prime(x) == true
            {
                y = y + 1;
            }
        }
        println!("{}",y);
        break;
    }
}

/*
$ time ./target/debug/prime_counter.exe
22044

real    1m32.153s
user    0m0.000s
sys     0m0.031s
*/