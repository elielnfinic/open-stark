pub fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    } else {
        let (d, x, y) = xgcd(b, a % b);
        return (d, y, x - (a / b) * y);
    }
}

pub struct FieldElement {
    value : i128, 
    field : Field
}

impl FieldElement {
    pub fn new(value : i128, field : Field) -> FieldElement{
        FieldElement{value, field}
    }

    pub fn _add(self, right : FieldElement) -> FieldElement{
        self.field.add(self, right)
    }

    pub fn _mul(self, right : FieldElement) -> FieldElement{
        self.field.multiply(self, right)
    }

    pub fn _sub(self, right : FieldElement) -> FieldElement{
        self.field.subtract(self, right)
    }

    pub fn _mul(self, right : FieldElement) -> FieldElement{
        self.field.multiply(self, right)
    }

    pub fn _inv(self) -> FieldElement{
        self.field.inverse(self)
    }
}

pub struct Field {
    p : i32
}

impl Field {
    pub fn init() -> Field{
        let p = 1 + 407 * ( 1 << 119);
        Field{ p }
    }

    pub fn new(p : i32) -> Field{
        Field{ p }
    }

    pub fn zero(&self) -> FieldElement{
        FieldElement::new(0, self.p)
    }

    pub fn one(&self) -> FieldElement{
        FieldElement::new(1, self.p)
    }

    pub fn multiply(&self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::new((left.value * right.value) % (self.p as i128), self.p)
    }

    pub fn add(&self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::new((left.value + right.value) % (self.p as i128), self.p)
    }

    pub fn subtract(&self, left : FieldElement, right : FieldElement) -> FieldElement{
        FieldElement::new((left.value - right.value) % (self.p as i128), self.p)
    }

    pub fn divide(&self, left : FieldElement, right : FieldElement) -> FieldElement{
        let (d, x, _) = xgcd(right.value as i64, self.p as i64);
        if d != 1 {
            panic!("{} is not invertible", right.value);
        }
        FieldElement::new((left.value * x as i128) % (self.p as i128), self.p)
    }

    pub fn inverse(&self, operand : FieldElement) -> FieldElement{
        self.divide(self.one(), operand)
    }

    pub fn pow(&self, left : FieldElement, right : i128) -> FieldElement{
        let mut n = right;
        let mut result = self.one();
        let mut current = left;
        while n > 0 {
            if n % 2 == 1 {
                result = self.multiply(result, current);
            }
            current = self.multiply(current, current);
            n /= 2;
        }
        result
    }

    pub fn negate(&self, operand : FieldElement) -> FieldElement{
        FieldElement::new(-operand.value, self.p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xgcd() {
        assert_eq!(xgcd(8, 12), (4, -1, 1));
        assert_eq!(xgcd(36163, 21199), (1247, -7, 12));
    }
}


