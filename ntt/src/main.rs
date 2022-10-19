static MOD:i64 = 998244353;
static G:i64 = 3;//generator of F_998244353
#[allow(dead_code)]
fn solve(read: &mut snio::Reader<std::io::StdinLock<'_>>) {
    let n = read.usize();
    let m = read.usize();
    let mut a:Vec<i64> = (0..n).map(|_| read.i64()).collect();
    let mut b:Vec<i64> = (0..m).map(|_| read.i64()).collect();
    let c = convolve(&mut a,&mut b);
    for i in 0..n + m - 1 {
        if i != n + m  - 2{
            print!("{} ",c[i]);
        }else {
            print!("{}\n",c[i]);
        }
    }
}
fn recursive_ntt(a:&mut Vec<i64>,inversion:i64) -> Vec<i64>{
    if a.len() == 1 {
        return a.to_vec(); 
    }
    let mut omega_n = {modpow(G,(MOD - 1)/a.len() as i64,MOD)};
    if inversion == -1 {
        omega_n = modpow(omega_n,MOD - 2,MOD);
    }
    let mut omega = 1;
    let mut a0 = Vec::new();
    let mut a1 = Vec::new();
    for i in (0..a.len()/2) {
        a0.push(a[2 * i]);
        a1.push(a[2 * i + 1]);
    }
    let y0 = recursive_ntt(&mut a0,inversion);
    let y1 = recursive_ntt(&mut a1,inversion);
    let mut y = vec![0;a.len()];
    for k in 0..a.len()/2 {
        y[k] = (y0[k] + omega * y1[k] % MOD) % MOD;
        y[k + a.len()/2] = (y0[k] - (omega * y1[k]) % MOD + MOD) % MOD;
        omega *= omega_n;
        omega %= MOD;
    }
    return y;
}
fn convolve(a:&mut Vec<i64>,b:&mut Vec<i64>) -> Vec<i64> {
    let n = (a.len() + b.len()).next_power_of_two();
    let mut ntt_a = vec![0;n];
    let mut ntt_b = ntt_a.clone();
    let mut inversion = 1;
    for i in 0..a.len() {
        ntt_a[i] = a[i] % MOD;
    }
    for i in 0..b.len() {
        ntt_b[i] = b[i] % MOD;
    }
    let a0 = recursive_ntt(&mut ntt_a,inversion);
    let b0 = recursive_ntt(&mut ntt_b,inversion);
    let mut ab = vec![0;n];
    for i in 0..n {
        ab[i] = (a0[i] * b0[i]) % MOD;
    }
    inversion = -1;
    let mut still = recursive_ntt(&mut ab,inversion);
    let mut res = vec![0;n];
    let inv_n = modpow(n as i64,MOD - 2,MOD);
    for i in 0..n {
        res[i] = (still[i] * inv_n) % MOD;
    }
    res
}

fn modpow(_n: i64, _t: i64, modulo: i64) -> i64 {
    let mut ret = 1i64;
    let mut n = _n;
    let mut t = _t;
    while t != 0 {
        if t & 1 == 1 {
            ret *= n;
        }
        ret %= modulo;
        t >>= 1;
        n *= n;
        n %= modulo;
    }
    ret
}
//use proconio::input;
fn main() {
    let t = std::io::stdin();
    let mut read = snio::Reader::new(t.lock());
    let n = 1;
    for _ in 0..n {
        solve(&mut read);
    }
}
 
#[allow(dead_code)]
pub mod snio {
    pub struct Reader<R: std::io::BufRead> {
        reader: R,
        buf: std::collections::VecDeque<String>,
    }
 
    impl<R: std::io::BufRead> Reader<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf: std::collections::VecDeque::new(),
            }
        }
        fn load(&mut self) {
            while self.buf.is_empty() {
                let mut s = String::new();
                let length = self.reader.read_line(&mut s).unwrap();
                if length == 0 {
                    break;
                }
                self.buf.extend(s.split_whitespace().map(|s| s.to_owned()));
            }
        }
        pub fn string(&mut self) -> String {
            self.load();
            self.buf.pop_front().unwrap_or_else(|| panic!("input ended"))
        }
        pub fn char(&mut self) -> char {
            let string = self.string();
            let mut chars = string.chars();
            let res = chars.next().unwrap();
            assert!(chars.next().is_none(), "invalid input!");
            res
        }
        pub fn chars(&mut self) -> Vec<char> {
            self.read::<String>().chars().collect()
        }
        pub fn read<T: std::str::FromStr>(&mut self) -> T
            where
                <T as ::std::str::FromStr>::Err: ::std::fmt::Debug,
        {
            self.string().parse::<T>().expect("Failed to parse the input.")
        }
    }
    macro_rules! definition_of_reader_of_numbers {
            ($($ty:tt,)*) => {
                impl <R:std::io::BufRead> Reader<R> {
                    $(
                    #[inline]
                    pub fn $ty (&mut self) -> $ty {
                        self.read::<$ty>()
                    }
                    )*
                }
            }
        }
    definition_of_reader_of_numbers! {
        u8,u16,u32,u64,u128,usize,
        i8,i16,i32,i64,i128,isize,
        f32,f64,
    }
}

const INF:i64 = 1i64 << 60;
