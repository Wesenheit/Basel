/*
    Functions used mainly used in mathematics on various finite fields.
 */

pub fn mod_pow(a:u64,x:u64,p:u64)->u64
{
    /*
        compute a^x mod p
     */
    let mut out:u64 = 1;
    let mut base:u64 = a % p;
    let mut exp:u64 = x;
    while exp > 0
    {
        if exp % 2 == 1
        {
            out = (out * base ) %p
        }
        exp = exp >> 1;
        base = ( base   *  base  ) %p;
    }
    return out;
}


pub fn legandre(a:u64,p:u64)->u64
{
    let ls = mod_pow(a,(p-1)/2,p);
    return ls
}


pub fn tonelli_shanks(a:u64,p:u64)->u64
{
    /*
        function used to find such number a that 
            x^2 mod p = a
        so square root of number using tonelli shanks algo.
     */
    if legandre(a,p) !=1
    {
        return 0;
    }
    else if a == 0
    {
        return 0;
    }
    else if p == 2
    {
        return 0;
    }
    else if p % 4 == 3
    {
        return mod_pow(a,(p+1)/4,p);
    }
    let mut s:u64 = p -1;
    let mut e:u64 = 0;
    while s%2 == 0
    {
        s/=2;
        e+=1;
    }
    let mut n:u64 = 2;
    while legandre(n,p) == 1
    {
        n += 1;
    }
    let mut t:u64;
    let mut m:u64;
    let mut gs:u64;
    let mut x:u64 = mod_pow(a,(s+1)/2,p);
    let mut b:u64 = mod_pow(a,s,p);
    let mut g:u64 = mod_pow(n,s,p);
    let mut r:u64 = e;
    let out:u64 = loop{
        t = b;
        m = 0;
        while t != 1 && m < r
        {
            t = mod_pow(t,2,p);
            m += 1;
        }
        if m == 0
        {
            break x;
        }
        gs = mod_pow(g,mod_pow(2,r-m-1,p),p);
        g = (gs * gs) % p;
        x = (x * gs) % p;
        b = (b * g) % p;
        r = m;
    };
    return out;
}

pub fn inverse(a:u64,p:u64)->u64
{
    let mut t:i64 = 0;
    let mut nt:i64 = 1;
    let mut r:i64 = p as i64;
    let mut nr:i64 = a as i64;
    let mut temp1:i64;
    let mut temp2:i64;
    let mut q:i64;
    while  nr != 0
    {
        q=r/nr;
        temp1=nr;
        temp2=r - q * nr;
        r=temp1;
        nr=temp2;
        temp1=nt;
        temp2=t - q * nt;
        t=temp1;
        nt=temp2;
    }
    if r > 1 {return 0};
    println!("{}",t);
    return (t % (p as i64)) as u64
}