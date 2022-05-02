use derivative::Derivative;

static ACC_RATE: f64 = 3.5;
static ACC_RATE_EMPTY: f64 = 2.5;
static ACC_RATE_FULL: f64 = 1.0;
static DEC_RATE: f64 = 7.0;
static DEC_RATE_EMPTY: f64 = 5.0;
static DEC_RATE_FULL: f64 = 2.0;

pub trait VehicleTraits {
    fn get_type(&self) -> &str;
    fn get_current_speed(&self) -> f64;
    fn get_desired_speed(&self) -> f64;
    fn set_desired_speed(&mut self, speed: f64);
    fn accelerate(&mut self, seconds_delta: f64);
    fn decelerate(&mut self, seconds_delta: f64);
    fn set_current_speed(&mut self, speed: f64);

    fn update_speed(&mut self, seconds: f64) {
        if self.get_current_speed() > self.get_desired_speed() {
            self.decelerate(seconds);
        } else {
            self.accelerate(seconds);
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Car {
    #[derivative(Default(value = "0.0"))]
    pub current_speed: f64,
    #[derivative(Default(value = "0.0"))]
    pub desired_speed: f64,
}

impl VehicleTraits for Car {
    fn get_type(&self) -> &str {
        "Car"
    }

    fn get_current_speed(&self) -> f64 {
        self.current_speed
    }

    fn get_desired_speed(&self) -> f64 {
        self.desired_speed
    }

    fn set_desired_speed(&mut self, speed: f64) {
        self.desired_speed = speed
    }

    fn accelerate(&mut self, seconds_delta: f64) {
        self.set_current_speed(self.current_speed + ACC_RATE * seconds_delta);
    }

    fn decelerate(&mut self, seconds_delta: f64) {
        self.set_current_speed(self.current_speed - DEC_RATE * seconds_delta)
    }

    fn set_current_speed(&mut self, speed: f64) {
        if self.current_speed <= speed {
            self.current_speed = speed.min(self.desired_speed)
        } else {
            self.current_speed = speed.max(self.desired_speed)
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct Truck {
    #[derivative(Default(value = "0.0"))]
    pub current_speed: f64,
    #[derivative(Default(value = "0.0"))]
    pub desired_speed: f64,
    #[derivative(Default(value = "0"))]
    pub load_weight: u32,
}

impl VehicleTraits for Truck {
    fn get_type(&self) -> &str {
        "Truck"
    }
    fn get_current_speed(&self) -> f64 {
        self.current_speed
    }
    fn get_desired_speed(&self) -> f64 {
        self.desired_speed
    }
    fn set_desired_speed(&mut self, speed: f64) {
        self.desired_speed = speed
    }

    fn accelerate(&mut self, seconds_delta: f64) {
        let constant = if self.load_weight <= 5 {
            ACC_RATE_EMPTY
        } else {
            ACC_RATE_FULL
        };
        self.set_current_speed(self.current_speed + constant * seconds_delta);
    }

    fn decelerate(&mut self, seconds_delta: f64) {
        let constant = if self.load_weight <= 5 {
            DEC_RATE_EMPTY
        } else {
            DEC_RATE_FULL
        };
        self.set_current_speed(self.current_speed - constant * seconds_delta);
    }

    fn set_current_speed(&mut self, speed: f64) {
        if self.current_speed <= speed {
            self.current_speed = speed.min(self.desired_speed)
        } else {
            self.current_speed = speed.max(self.desired_speed)
        }
    }
}
