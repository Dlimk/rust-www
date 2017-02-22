// Ez a kód szerkeszthető és futtatható!
fn main() {
    // Egy egyszerű számológép egész számokhoz:
    // `+` vagy `-` jelentése: 1 hozzáadása vagy kivonása
    // `*` vagy `/` jelentése: szorzás vagy osztás 2-vel

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* minden más figyelmen kívül hagyása */ }
        }
    }

    println!("A(z) \"{}\" program a(z) {} értéket számolja ki.",
              program, accumulator);
}
