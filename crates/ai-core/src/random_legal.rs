use engine_core::{
    ActionChoice, ActionNode, ActionPath, ActionTree, DeterministicRng, Diagnostic, Seed, SeededRng,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RandomLegalBot {
    pub seed: Seed,
}

impl RandomLegalBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(&self, tree: &ActionTree) -> Result<ActionPath, Diagnostic> {
        let mut rng = SeededRng::from_seed(self.seed);
        self.select_action_with_rng(tree, &mut rng)
    }

    pub fn select_action_with_rng(
        &self,
        tree: &ActionTree,
        rng: &mut dyn DeterministicRng,
    ) -> Result<ActionPath, Diagnostic> {
        let paths = legal_paths(tree);
        let index = rng.next_index(paths.len()).ok_or_else(|| Diagnostic {
            code: "no_legal_actions".to_owned(),
            message: "no legal action is available".to_owned(),
        })?;

        Ok(paths[index].clone())
    }
}

pub fn legal_paths(tree: &ActionTree) -> Vec<ActionPath> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn collect_paths(node: &ActionNode, prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    for choice in &node.choices {
        collect_choice(choice, prefix.clone(), paths);
    }
}

fn collect_choice(choice: &ActionChoice, mut prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    prefix.push(choice.segment.clone());
    if let Some(next) = &choice.next {
        collect_paths(next, prefix, paths);
    } else {
        paths.push(ActionPath { segments: prefix });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{ActionChoice, ActionTree, FreshnessToken};

    fn tree() -> ActionTree {
        ActionTree::flat(
            FreshnessToken(0),
            vec![
                ActionChoice::leaf("a", "A", "A"),
                ActionChoice::leaf("b", "B", "B"),
                ActionChoice::leaf("c", "C", "C"),
            ],
        )
    }

    #[test]
    fn same_seed_selects_same_action() {
        let bot = RandomLegalBot::new(Seed(7));

        assert_eq!(bot.select_action(&tree()), bot.select_action(&tree()));
    }

    #[test]
    fn selected_action_is_from_legal_paths() {
        let bot = RandomLegalBot::new(Seed(9));
        let tree = tree();
        let selected = bot.select_action(&tree).expect("action selected");
        let paths = legal_paths(&tree);

        assert!(paths.contains(&selected));
    }

    #[test]
    fn empty_tree_returns_diagnostic() {
        let bot = RandomLegalBot::new(Seed(1));
        let tree = ActionTree::flat(FreshnessToken(0), Vec::new());
        let diagnostic = bot.select_action(&tree).expect_err("empty tree rejected");

        assert_eq!(diagnostic.code, "no_legal_actions");
    }
}
