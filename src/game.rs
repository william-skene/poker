/// A generic player of a game
pub trait GamePlayer<StateT, ActionT> {
    fn get_action(&self, state: &StateT, actions: &Vec<ActionT>) -> ActionT;
}

/// A generic game engine
pub trait GameEngine<StateT, ActionT> {
    fn run(&mut self);
}
