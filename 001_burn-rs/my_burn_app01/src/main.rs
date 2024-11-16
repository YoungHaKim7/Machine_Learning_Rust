use burn::{backend::Wgpu, tensor::Tensor};

type Backend = Wgpu;

fn main() {
    let devide = Default::default();

    let tensor_1 = Tensor::<Backend, 2>::from_data([[2., 3.], [4., 5.]], &devide);
    let tensor_2 = Tensor::<Backend, 2>::ones_like(&tensor_1);

    println!("{}", tensor_1 + tensor_2);
}
