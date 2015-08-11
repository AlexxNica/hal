use layer::{Layer};
use model::Model;
use af::{Array};

pub struct Sequential {
  layers: Vec<Box<Layer>>,
  optimizer: &'static str,
  loss: &'static str,
}

impl Model for Sequential {
  fn new(optimizer: &'static str, loss: &'static str) -> Sequential {
    Sequential {
      layers: Vec::new(),
      loss: loss,
      optimizer: optimizer,
    }
  }

  fn add(&mut self, layer: Box<Layer>) {
    self.layers.push(layer);
  }

  fn info(&self) {
    println!("loss : {}\noptimizer: {}", self.loss, self.optimizer);
  }

  fn forward(&self, activation: &Array) -> Array{
    let mut a = self.layers[0].forward(activation);
    for i in 1..self.layers.len() {
      a = self.layers[i].forward(&a);
    }
    a
  }

  fn backward(&self, target: &Array, train: bool){
    let y = self.layers.last().unwrap().get_activation();
    let mut diff = target - y;
    let mut grads = optimizer.grads(y);

    for i in (0..self.layers.len()).rev() {
      diffs = self.layers[i].backward(&diff, &grads, train);
      grads = optimizer.grads(self.layers[i].get_activation());
    }
  }
}
