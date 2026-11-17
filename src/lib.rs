//! Hierarchical gating for long-context sparse attention (v2 method).

pub struct HierarchicalGate {
    pub levels: usize,
    pub width: usize,
}

impl HierarchicalGate {
    pub fn score(&self, token: &[f32], level: usize) -> f32 {
        token.iter().take(self.width).map(|x| x * level as f32).sum()
    }
}
