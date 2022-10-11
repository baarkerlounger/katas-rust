pub struct Map {
    x_origin: usize,
    y_origin: usize,
    x_limit: usize,
    y_limit: usize,
}

impl Map {
    pub fn new(x_origin: usize, y_origin: usize, x_limit: usize, y_limit: usize) -> Self {
        Map {
            x_origin,
            y_origin,
            x_limit,
            y_limit,
        }
    }

    pub fn get_x_origin(&self) -> usize {
        self.x_origin
    }

    pub fn get_y_origin(&self) -> usize {
        self.y_origin
    }

    pub fn get_x_limit(&self) -> usize {
        self.x_limit
    }

    pub fn get_y_limit(&self) -> usize {
        self.y_limit
    }
}
