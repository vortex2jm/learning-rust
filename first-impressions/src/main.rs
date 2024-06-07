fn main() {
    // A variável não precisa ser mutável para substituir seu valor
    let x = 5;
    let x = x + 1;
    println!("{}", x);

    // Podemos reutilizar o mesmo nome para armazenar diferentes valores
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // ============================================ //
    // Scalar
    let a: i8 = 10; //
    let b: f64 = 2.2;
    let c: i128 = 10_000;
    let d: bool = true;
    let e: char = 'p';
    println!("{a}, {b}, {c}, {d}, {e}");

    // ============================================ //
    // Compound
    let tup: (i16, f32, bool) = (10, 3.3, false);
    let (x, y, z) = tup;
    println!("Value of x = {x}");

    let g = tup.1;
    println!("Second value of tuple = {g}")

}
