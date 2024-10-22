use candle_core::{Device, Result, Tensor};
use candle_nn::Module;
use candle_nn::ops;

pub fn softmax_naive(t: &Tensor) -> Result<Tensor> {
    // naive implementation of softmax
    let xp = t.exp()?;
    let sxp = xp.sum_keepdim(0)?;
    xp.broadcast_div(&sxp.transpose(0, 1)?)
}

pub struct SelfAttentionV1 {
    d_out: usize,
    wq: Tensor,
    wv: Tensor,
    wk: Tensor,
}

impl SelfAttentionV1 {
    pub fn new(d_in: usize, d_out: usize) -> SelfAttentionV1 {
        let wq = Tensor::randn(0f64, 1f64, (d_in, d_out), &Device::Cpu).unwrap();
        let wv = Tensor::randn(0f64, 1f64, (d_in, d_out), &Device::Cpu).unwrap();
        let wk = Tensor::randn(0f64, 1f64, (d_in, d_out), &Device::Cpu).unwrap();
        SelfAttentionV1 { d_out, wq, wv, wk }
    }
}

impl Module for SelfAttentionV1 {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        let q = xs.matmul(&self.wq)?;
        let k = xs.matmul(&self.wk)?;
        let v = xs.matmul(&self.wv)?;

        let tmp = q.matmul(&k.transpose(0, 1).unwrap())?;
        let tmp = ops::softmax(&tmp, 1)?;
        let tmp = (tmp / (self.d_out as f64).powf(0.5))?;
        let out = tmp.matmul(&v).unwrap();
        Ok(out)
    }
}
