 pub struct RandomGenerator {
       pub value: u8,
    }

    impl RandomGenerator {
        pub fn new(value: u8) -> Self {
            Self {
                value
            }
        }
    }