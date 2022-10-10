use num_complex::*;
fn recursive_fft(a:&mut Vec<Complex::<f64>>,inversion:Complex::<f64>) -> Vec<Complex::<f64>>{
    if a.len() == 1 {
        return a.to_vec(); 
    }
    let two = Complex::<f64>{re:2.0,im:0.0};
    let i = Complex::<f64>::i();
    let n_c = Complex::<f64>{re:a.len() as f64,im:0.0};
    let PI:Complex::<f64> = (Complex::<f64>{re:-1.0,im:0.0}).acos();
    let omega_n = (inversion * two * PI * i /n_c).exp();
    let mut omega = Complex::<f64>{re:1.0,im:0.0};
    let mut a0 = Vec::new();
    let mut a1 = Vec::new();
    for i in (0..a.len()/2) {
        a0.push(a[2 * i]);
        a1.push(a[2 * i + 1]);
    }
    let y0 = recursive_fft(&mut a0,inversion);
    let y1 = recursive_fft(&mut a1,inversion);
    let mut y = vec![Complex::<f64>{re:0.0,im:0.0};a.len()];
    for k in 0..a.len()/2 {
        y[k] = y0[k] + omega * y1[k];
        y[k + a.len()/2] = y0[k] - omega * y1[k];
        omega *= omega_n;
    }
    return y;
}

fn convolve(a:&mut Vec<i64>,b:&mut Vec<i64>) -> Vec<f64> {
    let n = (a.len() + b.len()).next_power_of_two();
    let mut fft_a = vec![Complex::<f64>::new(0.0,0.0);n];
    let mut fft_b = fft_a.clone();
    let mut inversion = Complex::<f64>{re:1.0,im:0.0};
    for i in 0..a.len() {
        (fft_a[i]).re = a[i] as f64;
    }
    for i in 0..b.len() {
        (fft_b[i]).re = b[i] as f64;
    }
    let a0 = recursive_fft(&mut fft_a,inversion);
    let b0 = recursive_fft(&mut fft_b,inversion);
    let mut ab = vec![Complex::<f64>{re:0.0,im:0.0};n];
    for i in 0..n {
        ab[i] = a0[i] * b0[i];
    }
    inversion.re = -1.0;
    let mut still_complex = recursive_fft(&mut ab,inversion);
    let mut res = vec![0.0;n];
    for i in 0..n {
        res[i] = ((still_complex[i]).re / n as f64).round();
    }
    res
}
