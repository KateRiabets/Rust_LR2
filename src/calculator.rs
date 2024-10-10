#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    InvalidInput,
}

#[derive(Default)]
pub struct Calculator {
    pub last_result: f64,
}

impl Calculator {
    pub fn apply_operator(&mut self, a: f64, b: f64, operator: &Option<String>) -> f64 {
        match operator.as_deref() {
            Some("+") => self.add(a, b),
            Some("-") => self.subtract(a, b),
            Some("*") => self.multiply(a, b),
            Some("/") => self.divide(a, b).unwrap_or(a), // Обработка деления на ноль
            _ => b, // Если оператор не найден, возвращаем b
        }
    }

    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a + b;
        self.last_result
    }

    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a - b;
        self.last_result
    }

    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a * b;
        self.last_result
    }

    pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, CalcError> {
        if b == 0.0 {
            Err(CalcError::DivisionByZero)
        } else {
            self.last_result = a / b;
            Ok(self.last_result)
        }
    }
}
