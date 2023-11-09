use rand::Rng;
use std::ops;

#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }
}

struct Qubit {
    alpha: Complex,
    beta: Complex,
}

impl Qubit {
    fn new() -> Qubit {
        Qubit {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    fn hadamard_gate(&mut self) {
        let factor = Complex::new(1.0 / 2.0f64.sqrt(), 0.0);
        let alpha = self.alpha;
        let beta = self.beta;

        self.alpha = (alpha + beta) * factor;
        self.beta = (alpha - beta) * factor;
    }

    fn cnot_gate(&mut self, target: &mut Qubit) {
        let alpha = self.alpha * target.alpha;
        let beta = self.beta * target.beta;

        target.alpha = alpha;
        target.beta = beta;
    }

    fn measure(&mut self) -> u8 {
        let probability = self.alpha.real * self.alpha.real + self.alpha.imag * self.alpha.imag;
        let random_num: f64 = rand::thread_rng().gen();
        if random_num < probability {
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
            0
        } else {
            self.alpha = Complex::new(0.0, 0.0);
            self.beta = Complex::new(1.0, 0.0);
            1
        }
    }
}

fn main() {
    let mut qubit1 = Qubit::new();
    let mut qubit2 = Qubit::new();
    let n_measurements = 10000;
    
    let mut counts = [0; 2];
    
    for _ in 0..n_measurements {
        qubit1.hadamard_gate();
        qubit1.cnot_gate(&mut qubit2);

        let measurement1 = qubit1.measure();
        let measurement2 = qubit2.measure();

        counts[measurement1 as usize] += 1;
    }

    let total_measurements = n_measurements as f64;

    for i in 0..2 {
        let probability = counts[i] as f64 / total_measurements;
        println!("Probability of outcome {}: {:.4}", i, probability);
    }
}
