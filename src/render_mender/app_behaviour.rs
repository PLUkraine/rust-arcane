use super::app_state::AppState;


pub trait AppBehaviour {
    fn update(&self, app_state: &AppState);
    fn render(&self, app_state: &AppState);
}
