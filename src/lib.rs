extern crate rustr;

use std::fmt::{self, Debug};
use rustr::feature::engine::*;
use rustr::rdll::unix64::SEXP;

#[derive(Debug, Clone)]
pub struct Quantiles {
    // 25th
    pub lower: f64,
    // 50th
    pub median: f64,
    // 75th
    pub upper: f64,
}

pub struct RList<'a, T: Debug + 'a>(&'a [T]);

impl<'a, T: Debug> fmt::Debug for RList<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = self.0;
        write!(f, "c(")?;
        for (i, val) in inner.iter().enumerate() {
            write!(f, "{:?}", val)?;
            if i != (inner.len() - 1) {
                write!(f, ",")?;
            }
        }
        write!(f, ")")
    }
}

pub fn calculate_quantiles(values: Vec<i32>, weights: Vec<u32>) -> Quantiles {
    let mut re = unsafe { REngine::init().unwrap() };

    let _: SEXP = re.eval("suppressMessages(library(reldist))").unwrap();
    let _: SEXP = re.eval(&format!("my_data <- {:?}", RList(&values)))
        .unwrap();
    let _: SEXP = re.eval(&format!("my_weights <- {:?}", RList(&weights)))
        .unwrap();
    let quantiles: Vec<f64> = re.eval(
        "unname(wtd.quantile(my_data, q = c(0.25, 0.5, 0.75), weight = my_weights))",
    ).unwrap();

    Quantiles {
        lower: quantiles[0],
        median: quantiles[1],
        upper: quantiles[2],
    }
}
