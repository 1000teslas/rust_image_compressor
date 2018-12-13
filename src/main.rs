mod square_matrix;
mod bits;
mod huffman;
mod compressor;
mod trie;
mod decoder;

use square_matrix::SquareMatrix;
use bits::{BitVec, BitString};
use huffman::HuffmanEncoder;
use compressor::Compressor;
use trie::Trie;
use decoder::Decoder;

fn main() {
    println!("Hello, world!");

    let len = 4;
    let mut m = SquareMatrix::new(len);

    let mut k = 1;

    for i in 0..len {
        for j in 0..len {
            m.set(i, j, k);
            k += 1;
        }
    }

    let r = m.diagonal_unwrap();
    println!("{:?}", r);

    let mut b = BitVec::new();
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(&s);
    let s = BitString::new("0010000000000101");
    println!("{:?}", s);
    b.push_bits(&s);
    println!("{:?}", b);

    println!("");

    let mut h = HuffmanEncoder::new(
        &[(String::from("a"), 0.2),
          (String::from("b"), 0.1),
          (String::from("c"), 0.25),
          (String::from("d"), 0.45),
         ]
        );

    let s = h.encode(2);
    println!("");

//    println!("{:#?}", h);
 //   println!("{:#?}", s);

    let mut h = HuffmanEncoder::new(
        &[('a', 0.2),
          ('s', 0.1),
          ('e', 0.25),
          ('r', 0.45),
         ]
        );

    let c = h.encode(2);
    println!("");

    println!("{:#?}", h);

    let sentence = "sssaersssersaersaerssssssssssssssr";
    let chars : Vec<_> = sentence.chars().collect();
    let v = c.compress(&chars);
    println!("{:?}", sentence);
    println!("{}", v);

    /*
    let iter = v.iter();
    for i in iter {
        print!("{}", i);
    }
    println!("\n");

    let mut t = Trie::new();
    t.insert(&[1, 1, 0], 's');
    t.insert(&[1, 0, 0], 'a');
    t.insert(&[0], 'r');
    t.insert(&[0, 1], 'e');
    println!("{:#?}", t);

    test_trie(&t);
    */

    let d = Decoder::new(&c);
    let r = d.decode(&v);
    println!("{:#?}", d);
    println!("{}", sentence);
    println!("{:?}", r);

}

fn test_trie(t : &Trie<char>) {
    let mut s = t;
    for i in &[1, 1, 0, 1, 0] {
        match s.get_node(*i) {
            Some(n) => s = n,
            None => break,
        }
    }

    println!("{:#?}", s);
}
