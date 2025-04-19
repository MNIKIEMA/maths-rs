use std::fmt;
use std::ops::{Add, Mul};

#[derive(PartialEq, Debug)]
pub struct Polynome {
    pub coefs: Vec<f32>,
}

pub struct Scalar {
    pub value: f32,
}

/// This struct defines a polynome of degre n
/// The coeficients are represented as a vector
/// A vec![1, 2, 1] is the polynome 1 + 2X + X^2
///

impl Polynome {
    pub fn get_degre(&self) -> usize {
        self.coefs.len() - 1
    }

    pub fn get_value(&self, value: f32) -> f32 {
        let mut res: f32 = 0.0;
        for (deg, coef) in self.coefs.iter().enumerate() {
            res += coef * value.powi(deg as i32)
        }
        res
    }

    pub fn scalar_dot(&self, value: f32) -> Self {
        Polynome {
            coefs: self.coefs.iter().map(|&x| x * value).collect(),
        }
    }

    pub fn add(&self, other: &Polynome) -> Polynome {
        let mut coefs = Vec::new();
        let max_len = self.get_degre().max(other.get_degre());
        for i in 0..max_len {
            let coef1 = self.coefs.get(i).unwrap_or(&0.0);
            let coef2 = other.coefs.get(i).unwrap_or(&0.0);
            coefs.push(coef1 + coef2);
        }
        Polynome { coefs: coefs }
    }

    pub fn multiply(&self, other: &Polynome) -> Polynome {
        let deg1 = self.get_degre();
        let deg2 = other.get_degre();
        let deg = deg1 + deg2 + 1;
        let mut res_coefs = vec![0.0; deg];
        for i in 0..deg {
            for j in 0..i + 1 {
                let coef1 = self.coefs.get(j).unwrap_or(&0.0);
                let coef2 = other.coefs.get(i - j).unwrap_or(&0.0);
                res_coefs[i] += coef1 * coef2;
            }
        }

        Polynome { coefs: res_coefs }
    }

    pub fn derivative(&self) -> Polynome {
        if self.coefs.len() <= 1 {
            return Polynome { coefs: vec![0.0] };
        }
        let derive_coef: Vec<f32> = self
            .coefs
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &coef)| i as f32 * coef)
            .collect();
        Polynome { coefs: derive_coef }
    }

    pub fn primitive(&self) -> Polynome {
        let mut coefs = vec![0.0];
        coefs.extend(
            self.coefs
                .iter()
                .enumerate()
                .map(|(i, &coef)| coef / (i as f32 + 1.0)),
        );
        Polynome { coefs }
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
            for j in 0..i + 1 {
                let coef1 = self.coefs.get(j).unwrap_or(&0.0);
                let coef2 = rhs.coefs.get(i - j).unwrap_or(&0.0);
                res_coefs[i] += coef1 * coef2;
            }
        }
        Polynome { coefs: res_coefs }
    }
}

impl Mul<Scalar> for Polynome {
    type Output = Polynome;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Polynome {
            coefs: self.coefs.iter().map(|&x| x * rhs.value).collect(),
        }
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

        Polynome {
            coefs: result_coefficients,
        }
    }
}

impl<'a, 'b> Mul<&'b Polynome> for &'a Polynome {
    type Output = Polynome;

    fn mul(self, other: &'b Polynome) -> Polynome {
        self.multiply(other)
    }
}

impl fmt::Display for Polynome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms = Vec::new();

        for (i, &coef) in self.coefs.iter().enumerate() {
            if coef == 0.0 {
                continue;
            }

            let abs = coef.abs();
            let term = match i {
                0 => format!("{:.1}", coef),
                1 => {
                    if abs == 1.0 {
                        format!("{}X", sign_str(coef))
                    } else {
                        format!("{}{:.1}*X", sign_str(coef), abs)
                    }
                }
                _ => {
                    if abs == 1.0 {
                        format!("{}X^{}", sign_str(coef), i)
                    } else {
                        format!("{}{:.1}*X^{}", sign_str(coef), abs, i)
                    }
                }
            };

            terms.push(term);
        }

        if terms.is_empty() {
            write!(f, "0")
        } else {
            // First term may have a leading "+" to remove
            let mut result = terms.join(" ");
            if result.starts_with('+') {
                result = result[1..].trim_start().to_string();
            }
            write!(f, "{}", result)
        }
    }
}

fn sign_str(coef: f32) -> &'static str {
    if coef >= 0.0 {
        "+ "
    } else {
        "- "
    }
}
