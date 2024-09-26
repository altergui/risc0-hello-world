use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();

    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a = 0;
    let mut b = 1;
    for _ in 0..input {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    // write public output to the journal
    env::commit(&b);
}
