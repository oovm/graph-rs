use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PureNode<M> {
    index: usize,
    metadata: M,
}

impl<M> Node for PureNode<M> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<M> PureNode<M> {
    pub fn new(index: usize, metadata: M) -> Self {
        Self { index, metadata }
    }
    pub fn as_ref(&self) -> PureNode<&M> {
        PureNode::new(self.index, &self.metadata)
    }
    /// Cast the inner reference to outer reference.
    ///
    /// # Safety
    ///
    /// The memory layout of `PureNode<&M>` and `&PureNode<M>` is the same, so we can safely cast between them.
    ///
    /// Here we leak the `index`, it's safe since it is `Copy`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use graph_types::PureNode;
    /// let ref_inner = PureNode::new(0, &0);
    /// let ref_outer = PureNode::re_ref(ref_inner);
    /// ```
    pub fn re_ref(node: PureNode<&M>) -> &PureNode<M> {
        unsafe { &*(&node as *const PureNode<&M> as *const PureNode<M>) }
    }
    /// Cast the inner mutable reference to outer mutable reference.
    ///
    /// # Safety
    ///
    /// The memory layout of `PureNode<&mut M>` and `&mut PureNode<M>` is the same, so we can safely cast between them.
    ///
    /// `index` is leaked and promoted, make sure you don't modify its value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use graph_types::PureNode;
    /// let ref_inner = PureNode::new(0, &mut 0);
    /// let ref_outer = PureNode::re_mut(ref_inner);
    /// ```
    pub fn re_mut(mut node: PureNode<&mut M>) -> &mut PureNode<M> {
        unsafe { &mut *(&mut node as *mut PureNode<&mut M> as *mut PureNode<M>) }
    }
}
