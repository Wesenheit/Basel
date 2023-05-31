pub fn is_prime(x:i64)->bool
{
    /* check in number is prime using rather simple search */
    if x<2
    {
        return false;
    }
    if x<4
    {
        return true;
    }
    if x%2==0 || x%3==0
    {
        return false;
    }
    let mut i:i64=5;
    while i<=(x as f64).sqrt() as i64 
    {
        if (x%i==0) || (x%(i+2)==0)
        {
            return false;
        }
        i+=6;
    }
    return true;
}
