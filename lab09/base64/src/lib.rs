/// Encodes in base64
/// 
/// # Example:
/// ```  
/// # use base64::encode;
/// assert_eq!(encode(&Vec::from("Ma")),"TWE=");
/// assert_eq!(encode(&Vec::from("Man")),"TWFu");
/// assert_eq!(encode(&Vec::from("M")),"TQ==");
/// 
/// ```
pub fn encode(input: &[u8]) -> String{
    let alphabet = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];
    let mut output = String::new();
    let mut combined = 0u32;
    let mut counter = 0;

    for byte in input{
        if counter < 2{
            combined = (*byte as u32) << ((2 - counter % 3) * 8) | combined;
            counter += 1;
        }
        else {
            combined = (*byte as u32) << ((2 - counter % 3) * 8) | combined;
            output.push(alphabet[((combined >> (3 * 6)) & 0b111_111) as usize]);
            output.push(alphabet[((combined >> (2 * 6)) & 0b111_111) as usize]);
            output.push(alphabet[((combined >> 6) & 0b111_111) as usize]);
            output.push(alphabet[(combined & 0b111_111) as usize]);
            counter = 0;
            combined = 0;
        }
    }
    if counter == 1 {
        output.push(alphabet[((combined >> (3 * 6)) & 0b111_111) as usize]);
        output.push(alphabet[(((combined >> (2 * 8)) & 0b0000_0011) << 4) as usize]);
        output.push('=');
        output.push('=');
    }
    if counter == 2{
        output.push(alphabet[((combined >> (3 * 6)) & 0b111_111) as usize]);
        output.push(alphabet[((combined >> (2 * 6)) & 0b111_111) as usize]);
        output.push(alphabet[(((combined >> 8) & 0b0000_1111) << 2) as usize]);
        output.push('=');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nopadding() {
        let input = Vec::from("Man");
        let result = encode(&input);
        assert_eq!(result,"TWFu");
    }
    #[test]
    fn padding_1(){
        let input = Vec::from("Ma");
        let result = encode(&input);
        assert_eq!(result,"TWE=");
    }

    #[test]
    fn padding_2(){
        let input = Vec::from("M");
        let result = encode(&input);
        assert_eq!(result,"TQ==");
    }
}
