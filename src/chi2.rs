use crate::gamma::gamma_p;

pub fn chi2_df(chi2: f64, nf: usize) -> f64 {
    let a = 0.5 * nf as f64;
    let x = 0.5 * chi2;
    gamma_p(a, x)
}

pub fn chi2pdf_df(chi2: f64, nf: usize) -> f64 {
    let a = 0.5 * nf as f64;
    let x = a * chi2;
    gamma_p(a, x)
}

pub fn chi2pdf_xq(q: f64, nf: usize) -> f64 {
    chi2_xq(q, nf) / nf as f64
}

pub fn chi2_xq(q: f64, nf: usize) -> f64 {
    let x1 = 0.0;
    let q1 = chi2_df(x1, nf);
    let mut x2 = 0.0;
    loop {
        x2 += nf as f64;
        let qt = chi2_df(x2, nf);
        if qt >= q {
            break;
        }
    }
    let q2 = chi2_df(x2, nf);
    let f = |x: f64| chi2_df(x, nf);
    fi1(f, q, x1, x2)
}

///C INVERSE OF THE FUNCTION F.
///C RESULT:     FI1=X SUCH THAT Y=F(X).
///C PRECISSION: EPS=1/10**8 can be changed, see below.
///C METHOD:     BISECTING INTERVAL, INITIAL RANGE [X1,X2] (X1<X2).
///C ASSUMPTION: F(X) MONOTON (IN- OR DECREASING) IN THE INTERVAL [X1,X2].
fn fi1<F>(f: F, y: f64, x1: f64, x2: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let eps = 1.0 / 10f64.powi(8);
    const ITERMAX: usize = 1000;
    let mut i = 0;

    let mut xx1 = x1;
    let mut xx2 = x2;
    let mut y1 = f(xx1);
    let mut y2 = f(xx2);
    if y1 > y2 {
        (xx1, xx2, y1, y2) = (xx2, xx1, y2, y1);
    }

    if y >= y1 && y <= y2 {
        for _ in 0..ITERMAX {
            let xx = 0.5 * (xx1 + xx2);
            let ff = f(xx);
            if ff <= y {
                xx1 = xx;
            } else {
                xx2 = xx;
            }
            if (xx2 - xx1).abs() <= eps {
                return 0.5 * (xx2 + xx1);
            }
        }
    }

    println!("fi1: no convergence or y out of range !");
    println!("ITERMAX,EPS: {ITERMAX}, {eps}");
    println!("y1,y,y2: {y1}, {y}, {y2}");
    panic!();
}
