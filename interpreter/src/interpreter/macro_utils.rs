#[macro_export]
macro_rules! loop_controls {
    ($self:ident, $block: expr) => {
        let control = $self.eval_block(&$block);
        match control {
            ControlFlow::Break => break,
            ControlFlow::None | ControlFlow::Continue => {}

            _ => {
                $self.decrement_scope();
                return control;
            }
        }
    };
}
