use std::ops::{Mul, Add};


#[derive(PartialEq, Debug)]
struct Polynome{
    coefs: Vec<f32>
}

struct Scalar {
    value: f32
}

/// This struct defines a polynome of degre n
/// The coeficients are represented as a vector
/// A vec![1, 2, 1] is the polynome 1 + 2X + X^2
/// 


impl Polynome {
    fn get_degre(&self) -> usize {
        self.coefs.len() - 1
    }

    fn get_value(&self, value: f32) -> f32 {
        let mut res: f32 = 0.0;
        for (deg, coef) in self.coefs.iter().enumerate() {
            res += coef * value.powi(deg as i32)
        }
        res
    }

    fn scalar_dot(&self, value: f32) -> Self {
        Polynome {coefs: self.coefs.iter().map(|&x| x*value ).collect()}
    }

    fn add(&self, other: &Polynome) -> Polynome {
        let mut coefs = Vec::new();
        let max_len = self.get_degre().max(other.get_degre());
        for i in 0..max_len {
            let coef1 = self.coefs.get(i).unwrap_or(&0.0);
            let coef2 = other.coefs.get(i).unwrap_or(&0.0);
            coefs.push(coef1+coef2);
        }
        Polynome {coefs: coefs}
    }

    fn multiply(&self, other: &Polynome) -> Polynome {
        let deg1 = self.get_degre();
        let deg2 = other.get_degre();
        let deg = deg1 + deg2 + 1;
        let mut res_coefs = vec![0.0; deg];
        for i in 0..deg {
            for j in 0..i+1 {
                let coef1 = self.coefs.get(j).unwrap_or(&0.0);
                let coef2 = other.coefs.get(i - j).unwrap_or(&0.0);
                res_coefs[i] += coef1 * coef2;
            }
        }

        Polynome {coefs: res_coefs}
    }

    fn derivative(self) -> Polynome {
        if self.coefs.iter().count() == 1 {
            return Polynome { coefs: vec![0.0]};
        }
        let mut derive_coef = Vec::new();
        for (i, j) in self.coefs.iter().enumerate() {
            derive_coef.push(i as f32 * j);
        }
        Polynome { coefs: derive_coef }
    }

    fn primitive(self) {
        todo!()
    }
}

impl Mul for Polynome {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let deg1 = self.get_degre();
        let deg2 = rhs.get_degre();
        let deg = deg1 + deg2 + 1;
        let mut res_coefs = vec![0.0; deg];
        for i in 0..deg {
            for j in 0..i+1 {
                let coef1 = self.coefs.get(j).unwrap_or(&0.0);
                let coef2 = rhs.coefs.get(i - j).unwrap_or(&0.0);
                res_coefs[i] += coef1 * coef2;
            }
        }
        Polynome {coefs: res_coefs}
    }
}

impl Mul<Scalar> for Polynome {
    type Output = Polynome;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Polynome {coefs: self.coefs.iter().map(|&x| x*rhs.value ).collect()}
    }
    
}

impl Mul<Polynome> for Scalar {
    type Output = Polynome;

    fn mul(self, rhs: Polynome) -> Self::Output {
        Polynome {
            coefs: rhs.coefs.iter().map(|&x| self.value * x).collect(),
        }
    }
}

impl Add for Polynome {
    type Output = Polynome;

    fn add(self, other: Polynome) -> Polynome {
        let max_len = std::cmp::max(self.coefs.len(), other.coefs.len());
        let mut result_coefficients = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let coef1 = self.coefs.get(i).unwrap_or(&0.0);
            let coef2 = other.coefs.get(i).unwrap_or(&0.0);
            result_coefficients.push(coef1 + coef2);
        }

        Polynome {coefs: result_coefficients }
    }
}


fn main() {
    let poly = Polynome {
        coefs: vec![1.0, 2.0, 3.0]
    };

    let P= Polynome {
        coefs: vec![3.0, 2.0, 1.0]
    };

    println!("Degree: {}", poly.get_degre()); // Output: Degree: 3
    println!("Value at x=2: {}", poly.get_value(2.0)); // Output: Value at x=2: 17.0

    let is_eq = P == poly;


    print!("The Polynome {:?} and {:?} is it equam {}", P, poly, is_eq);
    let p3 = Polynome {coefs: vec![1.0, 2.0]};
    let p4 = Polynome {coefs: vec![1.0, 2.0, 3.0]};
    let product2 = p3.multiply(&p4);
    let scalar = Scalar {value: 2.0};
    let p_new = p3 * scalar;
    println!("The scalar prod is {:?}", p_new.coefs);
    println!("The product is {:?}", product2.coefs); //Output: [1.0, 4.0, 7.0, 6.0]
}
