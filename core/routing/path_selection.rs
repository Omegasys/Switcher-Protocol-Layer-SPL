use crate::routing::graph_router::GraphRouter;

pub struct PathSelector;

impl PathSelector {
    pub fn select_best_path(
        router: &GraphRouter,
        start: &str,
        goal: &str,
    ) -> Option<Vec<String>> {
        // Placeholder for AI + cost-based selection
        router.find_path(start, goal)
    }
}
