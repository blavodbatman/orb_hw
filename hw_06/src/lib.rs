pub struct MyStructure {
    int32: i32,
    float32: f32,
    tuple: (i32, i32),
    array: [i32; 3],
}

impl MyStructure {
    pub fn double_int32(&self) -> i32 {
        self.int32 + self.int32
    }

    pub fn double_int64(&self) -> i64 {
        self.int32 as i64 + self.int32 as i64
    }

    pub fn double_float32(&self) -> f32 {
        self.float32 + self.float32
    }

    pub fn double_float64(&self) -> f64 {
        self.float32 as f64 + self.float32 as f64
    }

    pub fn int_plus_float_to_float(&self) -> f64 {
        self.int32 as f64 + self.float32 as f64
    }

    pub fn int_plus_float_to_int(&self) -> i64 {
        self.int32 as i64 + self.float32 as i64
    }

    pub fn tuple_sum(&self) -> i32 {
        self.tuple.0 + self.tuple.1
    }

    pub fn array_sum(&self) -> i32 {
        self.array[0] + self.array[1] + self.array[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_structure = MyStructure {
            int32: 7,
            float32: 2.3,
            tuple: (15, 32),
            array: [11, 21, 31],
        };
        assert_eq!(my_structure.double_int32(), 14);
        assert_eq!(my_structure.double_int64(), 14);
        assert_eq!(my_structure.int_plus_float_to_int(), 9);
        assert!((my_structure.double_float32() - 4.6).abs() <= f32::EPSILON);
        assert!((my_structure.double_float64() - 4.6).abs() <= f32::EPSILON.into());
        assert!((my_structure.int_plus_float_to_float() - 9.3).abs() <= f32::EPSILON.into());
        assert_eq!(my_structure.tuple_sum(), 47);
        assert_eq!(my_structure.array_sum(), 63);
    }
}
