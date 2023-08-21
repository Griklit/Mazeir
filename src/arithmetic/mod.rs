pub trait DepthFirst {
    fn depth_first(&mut self, seed: Option<[u8; 16]>);
}