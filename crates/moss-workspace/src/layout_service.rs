pub struct EditorPartState {}

pub struct SidebarPartState {
    preferred_size: usize,
    is_visible: bool,
}

pub struct PanelPartState {
    preferred_size: usize,
    is_visible: bool,
}

pub struct LayoutState {
    editor_part_state: Option<EditorPartState>,
    sidebar_part_state: Option<SidebarPartState>,
    panel_part_state: Option<PanelPartState>,
}

// pub struct LayoutService {
//     layout_state_store: LayoutStateStore,
// }

// impl LayoutService {
//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn get_sidebar_part_state(&self) -> Option<SidebarPartState> {
//         todo!()
//     }

//     pub fn get_editor_part_state(&self) -> Option<EditorPartState> {
//         todo!()
//     }

//     pub fn get_panel_part_state(&self) -> Option<PanelPartState> {
//         todo!()
//     }
// }
