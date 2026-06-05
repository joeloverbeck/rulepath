//! Bot-facing placeholder contracts.

use engine_core::{ActionPath, Diagnostic, Viewer};

pub trait Bot {
    fn select_action(&self, viewer: &Viewer) -> Result<ActionPath, Diagnostic>;
}
#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::SeatId;

    struct FirstActionBot;

    impl Bot for FirstActionBot {
        fn select_action(&self, _viewer: &Viewer) -> Result<ActionPath, Diagnostic> {
            Ok(ActionPath {
                segments: vec!["first".to_owned()],
            })
        }
    }

    #[test]
    fn bot_trait_can_return_an_action_path() {
        let viewer = Viewer {
            seat_id: Some(SeatId("seat-a".to_owned())),
        };
        let action = FirstActionBot.select_action(&viewer).unwrap();

        assert_eq!(action.segments, vec!["first"]);
    }
}
