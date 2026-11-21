//! Toolbar items and commands.

use crate::components::menu::MenuItem;
use crate::core::AsyncAction;
use leptos::prelude::*;
use operator_ui_tokens::Icon as IconData;

/// What a toolbar command does.
#[derive(Clone)]
pub(crate) enum Run {
    Click(Callback<()>),
    Async(AsyncAction),
}

/// One command of a [`Toolbar`](super::Toolbar): an icon button in the bar, a menu row when it overflows.
#[derive(Clone)]
pub struct ToolbarItem {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) icon: IconData,
    pub(crate) run: Option<Run>,
    pub(crate) enabled: bool,
    pub(crate) danger: bool,
    pub(crate) separated: bool,
    pub(crate) overflow: bool,
}

impl ToolbarItem {
    /// Command with an id, a label (tooltip, accessible name, and menu text), and an icon.
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: IconData) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
            run: None,
            enabled: true,
            danger: false,
            separated: false,
            overflow: false,
        }
    }

    /// Runs `callback` when the command is chosen.
    pub fn on_click(mut self, callback: Callback<()>) -> Self {
        self.run = Some(Run::Click(callback));
        self
    }

    /// Runs an async action.
    /// In the bar the button shows its states; from the menu the overflow button shows them.
    pub fn on_async(mut self, action: AsyncAction) -> Self {
        self.run = Some(Run::Async(action));
        self
    }

    /// Disabled command.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Destructive command, shown in the error color in the menu.
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// Starts a new group: a separator in the bar, a divider in the menu.
    pub fn separated(mut self) -> Self {
        self.separated = true;
        self
    }

    /// Always lives in the overflow menu, for rarely used commands.
    pub fn overflow(mut self) -> Self {
        self.overflow = true;
        self
    }

    pub(crate) fn menu_item(&self, separated: bool) -> MenuItem {
        let mut item = MenuItem::new(self.id.clone(), self.label.clone()).icon(self.icon);
        if self.danger {
            item = item.danger();
        }
        if !self.enabled {
            item = item.disabled();
        }
        if separated {
            item = item.separated();
        }
        item
    }
}
