use std::fmt::{self};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Relation {
    Le,
    Ge,
    Eq,
    Leq,
    Geq,
}

#[derive(Debug, PartialEq)]
pub enum Objective {
    Maximize,
    Minimize,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub vars: Vec<char>,
    pub coeffs: Vec<f64>,
    pub b: f64,
    pub rel: Relation,
}

#[derive(Debug)]
pub struct ObjectiveFn {
    pub vars: Vec<char>,
    pub coeffs: Vec<f64>,
    pub obj: Objective,
}

impl fmt::Display for ObjectiveFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _objtype = if self.obj == Objective::Maximize {
            match write!(f, "maximize ") {
                Err(e) => writeln!(f, "{:?}", e),
                _ => Ok(()),
            }
        } else {
            match write!(f, "minimize ") {
                Err(e) => writeln!(f, "{:?}", e),
                _ => Ok(()),
            }
        };

        let it = self.vars.iter().zip(self.coeffs.iter());
        for (i, (var, coeff)) in it.clone().enumerate() {
            let _term = match write!(f, "{}{}", coeff, var) {
                Err(e) => writeln!(f, "{:?}", e),
                _ => Ok(()),
            };
            if i < it.len() - 1 {
                let _plus = match write!(f, " + ") {
                    Err(e) => writeln!(f, "{:?}", e),
                    _ => Ok(()),
                };
            }
        }
        write!(f, "")
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rel = match self {
            Relation::Le => "<=",
            Relation::Ge => ">=",
            Relation::Eq => "=",
            Relation::Leq => "<=",
            Relation::Geq => ">=",
        };
        write!(f, "{}", rel)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let it = self.vars.iter().zip(self.coeffs.iter());
        for (i, (var, coeff)) in it.clone().enumerate() {
            let _term = match write!(f, "{}{}", coeff, var) {
                Err(e) => writeln!(f, "{:?}", e),
                _ => Ok(()),
            };
            if i < it.len() - 1 {
                let _plus = match write!(f, " + ") {
                    Err(e) => writeln!(f, "{:?}", e),
                    _ => Ok(()),
                };
            }
        }
        let _rel = match write!(f, " {} 0", self.rel) {
            Err(e) => writeln!(f, "{:?}", e),
            _ => Ok(()),
        };
        write!(f, "")
    }
}


