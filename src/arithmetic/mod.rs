// mod depth_first;

// pub use self::depth_first::DepthFirst;

pub trait DepthFirst {
    fn depth_first(&mut self, seed: u64);
}