fn gcd(m: i128, n: i128) -> i128 {
  if m == 0 {
    n.abs()
  } else {
    gcd(n % m, m)
  }
}

fn main() {
  // Two random prime numbers
  let p: i128 = 3;
  let q: i128 = 7;

  // First part of public key:
  let n: i128 = p*q;

  // Finding other part of public key.
  // e stands for encrypt
  let phi: i128 = (p-1)*(q-1);

  let e = (2..).find(|&x| gcd(x as i128, phi) == 1).unwrap();

  // Private key (d stands for decrypt)
  // choosing d such that it satisfies
  // d*e = 1 + k * totient
  let k: i128 = 2;  // A constant value
  let d: i128 = (1 + (k * phi))/e as i128;

  // Message to be encrypted
  let msg: i128 = 20;
  println!("Message data = {}", msg);
  // Encryption c = (msg ^ e) % n
  let c: i128 = (i128::pow(msg, e) % n) + n;
  println!("\nEncrypted data = {}", c);

  // Decryption m = (c ^ d) % n
  let m: i128 = (((i128::pow(c, d as u32)) % n) +n) %n;

  println!("Original Message Sent = {} ", m);
}
