mod state;
mod state_types;
mod trait_objects;

#[cfg(test)]
mod tests {
    use crate::trait_objects;
    use crate::state;
    use crate::state_types;

    #[test]
    fn trait_objects() {
        trait_objects::run();
    }

    #[test]
    fn state_pattern() {
        state::run();
    }

    #[test]
    fn state_types_pattern() {
        state_types::run();
    }
}
