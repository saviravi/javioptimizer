mod representation;
mod simplex;

use crate::representation::{Constraint, ObjectiveFn, Objective, Relation};

fn convert_to_std_form(objfn: ObjectiveFn, _constraints: Vec<Constraint>) {
    println!("{objfn}");
    let new_objfn = convert_objfn(objfn);
    let (new_objfn, new_constraints) = replace_equality(new_objfn, _constraints);
    println!("{new_objfn}");
    for c in new_constraints {
        println!("{:?}", c);
    }
}

fn convert_objfn(objfn: ObjectiveFn) -> ObjectiveFn {
    // convert min to max
    if objfn.obj == Objective::Minimize {
        // invert coefficients
        let new_coeffs: Vec<f64> = objfn.coeffs.into_iter().map(|x| (-1.0 * x)).collect();
        let new_objfn = ObjectiveFn {
            vars: objfn.vars,
            coeffs: new_coeffs,
            obj: Objective::Maximize,
        };
        return new_objfn;
    } else {
        return objfn;
    }
}


fn replace_equality(objfn: ObjectiveFn, constraints: Vec<Constraint>) -> (ObjectiveFn, Vec<Constraint>) {
    let mut new_constraints: Vec<Constraint> = vec![];
    for constraint in constraints {
        if constraint.rel == Relation::Eq {
            let new_constraint = Constraint {
                vars: constraint.clone().vars,
                coeffs: constraint.clone().coeffs,
                b: constraint.clone().b,
                rel: Relation::Leq,
            };
            new_constraints.push(new_constraint);
            let new_constraint = Constraint {
                vars: constraint.clone().vars,
                coeffs: constraint.clone().coeffs,
                b: constraint.clone().b,
                rel: Relation::Geq,
            };
            new_constraints.push(new_constraint);
        } else {
            new_constraints.push(constraint);
        }
    }
    return (objfn, new_constraints);
}

fn add_nonneg_constraint(objfn: ObjectiveFn, constraints: Vec<Constraint>) -> (ObjectiveFn, Vec<Constraint>) {
    // check which variables have non-negativity constraints
    let vars = objfn.vars.clone();
    // let mut nonneg_vars: Vec<char> = vec![];
    for (i, var) in vars.iter().enumerate() {
        for constraint in &constraints {
            match constraint.rel {
                Relation::Geq => {
                    // variable exists in constraint
                    if constraint.coeffs[i] != 0.0 {
                        let idx = constraint.vars.iter().position(|&x| x == *var).unwrap();
                        if constraint.coeffs[idx] != 0.0 {
                            // nonneg_vars.push(var);
                            break;
                        }
                    }
                },
                _ => continue,
                
            }
            // if constraint.vars.contains(&var) {
            //     let idx = constraint.vars.iter().position(|&x| x == var).unwrap();
            //     if constraint.coeffs[idx] != 0.0 {
            //         nonneg = false;
            //         break;
            //     }
            // }
        }
    }

    return (objfn, constraints);
}

fn main() {
    let var_names = vec!['x', 'y'];
    let c1 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![0.0, 0.0],
        b: 0.0,
        rel: Relation::Eq,
    };
    let c2 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![1.0, 1.0],
        b: 0.0,
        rel: Relation::Eq,
    };
    let c3 = Constraint {
        vars: var_names.clone(),
        coeffs: vec![2.0, 1.0],
        b: 0.0,
        rel: Relation::Eq,
    };
    let objfn = ObjectiveFn {
        vars: var_names.clone(),
        coeffs: vec![-0.5, 0.1],
        obj: Objective::Minimize,
    };

    convert_to_std_form(objfn, vec![c1, c2, c3]);
    // let x: (ObjectiveFn, Vec<Constraint>) = replace_equality(objfn, vec![c1, c2, c3]);
    // println!("{:?}", x);
}
