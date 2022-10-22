fn garner(mod_and_remainder:&mut Vec<(i64,i64)>,modulo:i64) -> i64 {//mod_and_remainder[i] = (i回目のNTTの法,i回目のNTTにおける第x項)
    //厳密には引数のmoduloは何でもよい
    mod_and_remainder.push((modulo,0));//操作をしやすくするために末尾に要素を追加
    let mut coffs = vec![1;mod_and_remainder.len()];
    let mut constants = vec![0;mod_and_remainder.len()];
    for i in 0..mod_and_remainder.len() - 1 {
        let mut v = (mod_and_remainder[i].1 - constants[i]) * modpow(coffs[i],mod_and_remainder[i].0 - 2,mod_and_remainder[i].0) % mod_and_remainder[i].0;
        //coffs[i] * v + constants == mod_and_remainder[i].1 (mod mod_and_remainder[i].0)を解く
        if v < 0 {
            v += mod_and_remainder[i].0;
        }
        for j in i + 1..mod_and_remainder.len() {
            constants[j] += (coffs[j] * v) % mod_and_remainder[j].0;
            constants[j] %= mod_and_remainder[j].0;
            coffs[j] *= mod_and_remainder[i].0;
            coffs[j] %= mod_and_remainder[j].0;
        }
    }
    mod_and_remainder.pop();//追加した要素を削除
    constants[mod_and_remainder.len()]
}