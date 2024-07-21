// use tch::Tensor;

#[derive(Clone, Debug)]
pub struct Service {}

impl Service {
    pub fn new() -> Result<Service, Box<dyn std::error::Error>> {
        Ok(Service {})
    }

    pub fn predict(&self) -> Result<f64, Box<dyn std::error::Error>> {
        // let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
        // let t = t * 2;
        // t.print();

        return Ok(1.0);
    }
}
