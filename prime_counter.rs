fn is_prime(n: i32) -> bool
{
    for i in 2..=n/2
    {
        if n % i == 0
        {
            return false;
        }
}
    return true;
}
fn main()
{
    let r: i32 = 250000;

    let mut y: i32 = 0;
    for x in 2..=r
    {
        if is_prime(x) == true
        {
            y = y + 1;
        }
    }
    println!("{}",y);
}

/*
$ time ./target/debug/prime_counter.exe
22044

real    0m7.028s
user    0m0.000s
sys     0m0.046s
*/