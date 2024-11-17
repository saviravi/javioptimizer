use std::{convert, fmt};

#[derive(Debug, PartialEq)]
enum Relation {
    le,
    ge,
    eq,
    leq,
    geq,
}

#[derive(Debug, PartialEq)]
enum Objective {
    maximize,
    minimize,
}

#[derive(Debug)]
pub struct Constraint {
    vars: Vec<char>,
    coeffs: Vec<f64>,
    rel: Relation,
}

#[derive(Debug)]
pub struct ObjectiveFn {
    vars: Vec<char>,
    coeffs: Vec<f64>,
    obj: Objective,
}

impl fmt::Display for ObjectiveFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.obj == Objective::maximize {
            match write!(f, "maximize ") {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        } else {
            match write!(f, "minimize ") {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        };

        let it = self.vars.iter().zip(self.coeffs.iter());
        for (i, (var, coeff)) in it.clone().enumerate() {
            match write!(f, "{}{}", coeff, var) {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
            if i < it.len() - 1 {
                match write!(f, " + ") {
                    Err(e) => println!("{:?}", e),
                    _ => (),
                }
            }
        }
        write!(f, "")
    }
}

fn convert_to_std_form(objfn: ObjectiveFn, constraints: Vec<Constraint>) {
    println!("{objfn}");
    let new_objfn = convert_objfn(objfn);
    println!("{new_objfn}");
}

fn convert_objfn(objfn: ObjectiveFn) -> ObjectiveFn {
    // convert min to max
    if objfn.obj == Objective::minimize {
        // invert coefficients
        let new_coeffs: Vec<f64> = objfn.coeffs.into_iter().map(|x| (-1.0 * x)).collect();
        let new_objfn = ObjectiveFn {
            vars: objfn.vars,
            coeffs: new_coeffs,
            obj: Objective::maximize,
        };
        return new_objfn;
    } else {
        return objfn;
    }
}

fn add_nonneg_constraint() {
    
}

fn main() {
    let var_names = vec!['x', 'y'];
    let c1 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![0.0, 0.0],
        rel: Relation::eq,
    };
    let c2 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![1.0, 1.0],
        rel: Relation::eq,
    };
    let c3 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![2.0, 1.0],
        rel: Relation::eq,
    };
    let objfn = ObjectiveFn {
        vars: var_names,
        coeffs: vec![0.0, 0.1],
        obj: Objective::minimize,
    };

    convert_to_std_form(objfn, vec![c1, c2, c3]);
}
