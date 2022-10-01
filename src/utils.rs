 /*
 Takes an integer n and represents
    it as an integer in the range

    r = n % a

    for a odd:
        -(a-1)/2 < r <= (a-1)/2
    for a even:
        - a / 2  < r <= a / 2

*/
pub fn reduce_mod_pm(n: i64,a : i64)  -> i64 {
   let mut r: i64 = n % a;
    if (r > (a >> 1)){
    
        r -= a;
        return r;
    }
    
     

}