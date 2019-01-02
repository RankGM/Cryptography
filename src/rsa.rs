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
  let mut e: u32 = 2;
  let phi: i128 = (p-1)*(q-1);

  while (e as i128) < phi {
    if gcd(e as i128, phi) == 1 {
      break;
    }
    else {
      e += 1;
    }
  }

  // Private key (d stands for decrypt)
  // choosing d such that it satisfies
  // d*e = 1 + k * totient
  let k: i128 = 2;  // A constant value
  let d: i128 = (1 + (k * phi))/e as i128;

  // Message to be encrypted
  let msg: i128 = 20;
  println!("Message data = {}", msg);
  // Encryption c = (msg ^ e) % n
  let mut c: i128 = i128::pow(msg, e);
  // mod workaround
  c = (c % n) +n;
  println!("\nEncrypted data = {}", c);

  // Decryption m = (c ^ d) % n
  let mut m: i128 = i128::pow(c, d as u32);
  m = ((m % n) +n) % n;
  println!("Original Message Sent = {} ", m);
}
