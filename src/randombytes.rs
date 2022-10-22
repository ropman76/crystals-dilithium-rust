use ring::rand::SecureRandom;
//gerates the random bytes 
pub fn randombytes() ->  [u8; 10]
{
    let mut randoms: [u8; 10] = [0; 10];
    let sr = ring::rand::SystemRandom::new();
    sr.fill(&mut randoms);
    
    return randoms;
    
}