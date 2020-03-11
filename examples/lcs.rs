use ng_diff::{AsCharsIter, Hirschberg, HirschbergAlg};

fn main() {
    let lcs: String = Hirschberg::lcs(
        "ACCGGTCGAGTGCGCGGAAGCCGGCCGAA".chars_iter(),
        "GTCGTTCGGAATGCCGTTGCTCTGTAAA".chars_iter(),
    );
    println!("{}", lcs);
}
