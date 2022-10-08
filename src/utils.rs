 /*
 Takes an integer n and represents
    it as an integer in the range

    r = n % a

    for a odd:
        -(a-1)/2 < r <= (a-1)/2
    for a even:
        - a / 2  < r <= a / 2

*/
pub fn reduce_mod_pm(n: i64,a : i64) -> i64 {
   let mut r: i64 = n % a;
    if r > (a >> 1){
    
        r -= a;
        return r;
    } else {
        return 0
    }
    
     

}



#[cfg(test)]
mod tests {

    use super::*;
   /* #[test]
    fn testreduce() {
       let n:i64 = 10;
        let a:i64 = 20;

       let r :i64 =  reduce_mod_pm(n,a);

        let z   = n % a;
        let y = r % a;
        assert_eq!(z,y);
    }

*/ 



}