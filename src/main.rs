use std::env;

// In computer science, we leverage Scientific Notation to represent
// floating-point numbers, because it doesn't only have a fixed width, but also
// can encode a wide range of numbers. Each position within a number in scientific
// notaion is given a role:
// * A sign would be present for negative numbers
// * The mantissa, also known as the significand
// * The radix, also know as the base, is the value that is raised to the power of the exponent
// * The exponent describes the scale of the values
//
// According to IEEE 754, the memory layout of f32 is called binary32, which
// could be demonstrated like below figure.
// |31|30|29|28|27|26|25|24|23|22|21|20|19|18|17|16|15|14|13|12|11|10|9|8|7|6|5|4|3|2|1|0|
// |_| |_____________________| |________________________________________________________|
//  |             |                                     |
// Sign bit     Exponent                            Mantissa
// The radix is a constant value, 2, for binary numbers. Also the standard introduces another
// constant value for calculating the actual exponent, which is 127.
// Thus a floating-point number can be calculated by this equation:
//          n = -1**sign_bit * mantissa * Radix**(exponent-Bias)
//
// Note:
// * The floating-point numbers 0 and -0 are equal but have different bit patterns.
// * The NAN floating-point values have identical bit patterns but are not equal.

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let mut args = env::args();
    args.next();

    let n = args.next().expect("Please provide a floating-point number as a argument!");
    let n: f32 = n.parse::<f32>().expect("Invalid floating-point number!");

    let (sign_bits, exponent_bits, mantissa_bits) = parse(n);
    let (sign_real_num, exponent_real_num, mantissa_real_num) = decode(sign_bits, exponent_bits, mantissa_bits);
    let n_recalculated = recalculate(sign_real_num, exponent_real_num, mantissa_real_num);

    println!("{} is recalculated by its parts (sign, exponent, mantissa) -> {}", n, n_recalculated);
    println!("field     | as bits   | as real number");
    println!("sign      | {:01b}    | {}", sign_bits, sign_real_num);
    println!("exponent  | {:08b}    | {}", exponent_bits, exponent_real_num);
    println!("mantissa  | {:023b}   | {}", mantissa_bits, mantissa_real_num);
}

fn parse(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = bits & 0x7fffff;

    (sign, exponent, mantissa)
}

fn decode(sign_bits: u32, exponent_bits: u32, mantissa_bits: u32) -> (f32, f32, f32) {
    let sign_real_num = (-1.0_f32).powf(sign_bits as f32);

    let exponent_real_num = (exponent_bits as i32) - BIAS;
    let exponent_real_num = RADIX.powf(exponent_real_num as f32);

    let mut mantissa_real_num: f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = mantissa_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa_real_num += weight;
        }
    }

    (sign_real_num, exponent_real_num, mantissa_real_num)
}

fn recalculate(sign_real_num: f32, exponent_real_num: f32, mantissa_real_num: f32) -> f32 {
    sign_real_num * exponent_real_num * mantissa_real_num
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {

    }

    #[test]
    fn test_decode() {

    }

    #[test]
    fn test_recalculate() {

    }
}