use rand::{thread_rng, Rng};

/*
const POPULATION_SIZE:f64      =   50.0;
const INDIVIDUALS_SURVIVE:f64  =   20.0;
*/

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Const(f64),
    Arg,
}

const FUNCTION_PROBABILITY: f64 =    0.5;
const ARG_PROBABILITY:      f64 =    0.7;
const MIN_VAL:              f64 =  -20.0;
const MAX_VAL:              f64 =   20.0;

const MUTATE_ARG:           f64 =    0.1;
const MUTATE_NUM:           f64 =    0.6;
const MUTATE_FUN:           f64 =    0.2;
const MUTATE_MIN:           f64 =   -2.0;
const MUTATE_MAX:           f64 =    2.0;

impl Expr {
    fn eval(&self, arg: f64) -> f64 {
        match self {
            Add(e1, e2) => e1.eval(arg) + e2.eval(arg),
            Sub(e1, e2) => e1.eval(arg) - e2.eval(arg),
            Mul(e1, e2) => e1.eval(arg) * e2.eval(arg),
            Div(e1, e2) => e1.eval(arg) / e2.eval(arg),
            Sin(e1)     => e1.eval(arg).sin(),
            Cos(e1)     => e1.eval(arg).cos(),
            Tan(e1)     => e1.eval(arg).tan(),
            Const(n)    => *n,
            Arg         => arg,
        }
    }

    fn generate_with_depth(depth: u32) -> Expr {
        let mut rng = thread_rng();
        let p1: f64 = rng.gen();

        if depth < 1 {
            return {
                if p1 < ARG_PROBABILITY {
                    Arg
                }
                else {
                    Const(rng.gen_range(MIN_VAL, MAX_VAL))
                }
            }
        }

        if p1 < FUNCTION_PROBABILITY {
            let e1 = Box::new(Expr::generate_with_depth(depth - 1));
            match rng.gen_range(0u8, 7u8) {
                0 => Add(e1, Box::new(Expr::generate_with_depth(depth - 1))),
                1 => Sub(e1, Box::new(Expr::generate_with_depth(depth - 1))),
                2 => Mul(e1, Box::new(Expr::generate_with_depth(depth - 1))),
                3 => Div(e1, Box::new(Expr::generate_with_depth(depth - 1))),
                4 => Sin(e1),
                5 => Cos(e1),
                _ => Tan(e1),
            }
        }
        else if p1 < ARG_PROBABILITY {
            Arg
        }
        else {
            Const(rng.gen_range(MIN_VAL, MAX_VAL))
        }
    }

    fn is_fun(&self) -> bool {
        match self {
            Add(_, _) | Sub(_, _) | Mul(_, _) |
            Div(_, _) | Sin(_) | Cos(_) | Tan(_) => true,
            _ => false,
        }
    }

    fn is_arg(&self) -> bool {
        match self {
            Arg => true,
            _   => false,
        }
    }

    fn mutate_with_depth(&mut self, depth: u32) {
        if depth == 0 {
            return;
        }
        let mut rng = thread_rng();
        let p1: f64 = rng.gen();
        let d1 = depth - 1;

        if (p1 < MUTATE_FUN && self.is_fun()) ||
           (p1 < MUTATE_ARG && self.is_arg()) {
            *self = Expr::generate_with_depth(depth);
        }

        match self {
            Add(e1, e2) | Sub(e1, e2) |
            Mul(e1, e2) | Div(e1, e2)   => { e1.mutate_with_depth(d1); e2.mutate_with_depth(d1) },
            Sin(e1) | Cos(e1) | Tan(e1) => { e1.mutate_with_depth(d1); },
            Const(n)                    => if p1 < MUTATE_NUM {
                    *n += rng.gen_range(MUTATE_MIN, MUTATE_MAX);
                }
            _ => (),
        }
    }
}

use crate::Expr::*;

fn target_fun(x: f64) -> f64 {
    (1.46327*x*x).sin() + x/0.3423
}

const FITNESS_MIN:    f64   =   -7.0;
const FITNESS_MAX:    f64   =    7.0;
const FITNESS_POINTS: usize =  1000;
const FITNESS_STEP:   f64   = (FITNESS_MAX - FITNESS_MIN) / (FITNESS_POINTS - 1) as f64;

fn make_points<F>(f: F) -> Vec<f64>
  where F: Fn(f64) -> f64 {
    let mut result = Vec::with_capacity(FITNESS_POINTS);

    for x in 0..FITNESS_POINTS {
        result.push(f(FITNESS_MIN + FITNESS_STEP * x as f64));
    }
    result
}

const EXCEPTION_WEIGHT: f64 = 1000.0;

fn calc_fitness<F>(data: &Vec<f64>, f: F) -> f64
  where F: Fn(f64) -> f64 {
      let mut result = 0f64;
      for x in 0..FITNESS_POINTS {
          let fx = f(FITNESS_MIN + FITNESS_STEP * x as f64);
          if data[x] == std::f64::INFINITY || data[x] == std::f64::NEG_INFINITY {
              if data[x] != fx {
                  result += EXCEPTION_WEIGHT;
              }
          }
          else {
              result += (fx - data[x]).abs();
          }
      }
      result
  }

fn main() {
    let mut e1 = Expr::generate_with_depth(5);
    let mut e2 = Expr::generate_with_depth(5);

    let v_target = make_points(target_fun);

    println!("{:?}\n{:?}", e1, e2);

    println!("{}", calc_fitness(&v_target, |x| e1.eval(x)));
    println!("{}", calc_fitness(&v_target, |x| e2.eval(x)));

    for _ in 0..5 {
        e1.mutate_with_depth(5);
        e2.mutate_with_depth(5);
        println!("\n\n{:?}\n{:?}", e1, e2);
    }
}
