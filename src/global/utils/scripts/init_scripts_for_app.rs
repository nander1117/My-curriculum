use crate::global::utils::scripts::{
    active_interset_for_scroll_animation, come_back_please, language, Lang,
};
/// * ### Initialize all scripts of JS for the app
pub fn init_scripts_for_app() {
    language(Lang::Es);
    come_back_please();
    active_interset_for_scroll_animation();
}
