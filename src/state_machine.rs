pub trait StateTransition<C> {
    fn on_enter(&mut self);
    fn on_exit(&mut self);
    fn update(&mut self, game: &mut C) -> Option<Box<dyn StateTransition<C>>>;
}
