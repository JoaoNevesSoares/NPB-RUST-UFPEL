const T23: f64 = (1 << 23) as f64;
const R23: f64 = 1.0 / ((1 << 23) as f64);
const T46: f64 = T23 * T23;
const R46: f64 = R23 * R23;
use fma::fma;

pub fn randlc(x: &mut f64, a: f64) -> f64 {
    let (mut t1, t2, t3, t4, a1, a2, x1, x2, z) : (f64, f64, f64,f64, f64, f64, f64, f64, f64);
    //-------------------------------------------------------
    //Break A into two parts such that A = 2ˆ23 * A1 + A2.
    //-------------------------------------------------------
    t1 = R23 * a;
    a1 = t1.trunc();
    a2 = a - T23 *a1;
    //-------------------------------------------------------
    //break X into two parts such that X = 2ˆ23 * X1 + X2,
    //compute Z = A1*X2 + A2*X1 (mod 2ˆ23), and then
    //X = 2ˆ23 * Z + A2 * X2 (mod 2ˆ46)
    //-------------------------------------------------------
    t1 = R23 * (*x);
    x1 = t1.trunc();
    x2 = (*x) - T23 * x1;
    t1 = fma(a1, x2, a2 * x1);//t1 = fma(a1, x2,a2*x1);
    //t1 = a1 * x2 + a2 * x1;
    t2 = (R23 * t1).trunc();
    z = t1 - T23 * t2;
    t3 = fma(T23, z, a2 * x2);//t3 = fma(T23,z,a2*x2);
    //t3 = T23 * z + a2 * x2;
    t4 = (R46*t3).trunc();
    *x = t3 - T46 * t4;
    //*x = fma(*x,t3,-T46*t4);//*x = t3 - T46 * t4;
    return R46 * (*x);
}

pub fn vranlc(n: i32, x_seed: &mut f64, a: f64, y: &mut[f64]){//y: &mut Vec<f64>
    let (mut x,mut t1, mut t2, mut t3, mut t4, a1, a2, mut x1, mut x2, mut z) : (f64, f64, f64, f64,f64, f64, f64, f64, f64, f64);
    //-------------------------------------------------------
    //break A into two parts such that A = 2ˆ23 * A1 + A2
    //-------------------------------------------------------
    t1 = R23 * a;
    a1 = t1.trunc();//(t1 as i32) as f64;
    a2 = a - T23 *a1;
    x = *x_seed;
    //-------------------------------------------------------
    //generate N results. this loop is not vectorizable.
    //-------------------------------------------------------
    for i in 0..n as usize{
    //-------------------------------------------------------
    //break X into two parts such that X = 2ˆ23 * X1 + X2,
    //compute Z = A1*X2 + A2*X1 (mod 2ˆ23), and then
    //X = 2ˆ23 * Z + A2 * X2 (mod 2ˆ46)
    //------------------------------------------------------- 
        t1 = R23 * x;
        x1 = t1.trunc();//(t1 as i32) as f64;
        x2 = x - T23 * x1;
        t1 = fma(a1, x2, a2 * x1);
        //t1 = a1* x2 + a2 * x1;//fma(t1,a1*x2,a2*x1);//
        t2 = (R23 * t1).trunc();//((R23 * t1) as i32) as f64;
        z = t1 - T23 * t2;
        t3 = T23 * z + a2 * x2;//t3 = fma(T23,z, a2 * x2);//
        t4 = (R46 * t3).trunc();//((R46 * t3) as i32) as f64;
        x = t3 - T46 * t4;
        y[i] = R46 * x;
    }
    *x_seed = x;
}