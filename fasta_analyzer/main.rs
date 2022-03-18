fn main() {
// given a DNA sequence , count the ocuurence of A,G,T,C
// DNA _string = "AGTCTCTGATCTAGAATAGCCTAGCTATTTACTAGGATCTTTCGAGGGATGTCCCCTTTTCAAAAAAGGA"
// create string instance
// TIP: let s = String::from("hello");  // s comes into scope
// use find for each nucleotides, 
// TIP: let c = s.matches(t).count(); // count occurence of substring t in s
// find total len of DNA
// let len = s.len()
// print function
// println!("{}", some_string);
//
//


    let dna_string = "AGTCTCTGATCTAGAATAGCCTAGCTATTTACTAGGATCTTTCGAGGGATGTCCCCTTTTCAAAAAAGGA";
    let g_count = dna_string.matches("G").count();
    println!("{}", dna_string.len());
    println!("{}", g_count);
}
