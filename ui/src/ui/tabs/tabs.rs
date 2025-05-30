use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

use crate::appdata::tabs::{TabItem, TabItemManager};

#[component]
pub fn TabItemManagerUi() -> Element {
    let tabs = use_context::<Signal<TabItemManager>>();

    return rsx! {
        div {
            class: "flex border-b pt-1 px-1",
            for tab in tabs().tabs {
                TabItemUi { item: tab }
            }
        }
    };
}

#[component]
pub fn TabItemUi(item: TabItem) -> Element {
    let mut tab_manager = use_context::<Signal<TabItemManager>>();
    let is_current_tab = tab_manager().get_current_tab().is_some()
        && tab_manager().get_current_tab().unwrap().id == item.id;

    let border_class = if is_current_tab {
        "border-b-blue-400!"
    } else {
        "border-b-transparent!"
    };

    // i just had to do this 
    let cloned_item = item.clone();

    return rsx! {
        button {
            class: "p-1 pb-2.5 border-b-2 {border_class} flex items-center justify-between gap-2 group",
            onclick: move |_| {
                let tman = &mut tab_manager.write();
                tman.set_current_tab(cloned_item.id.clone());
            },
            span {
                class: "text-sm",
                "{item.name}"
            }
            button {
                onclick: move |event| {
                    event.stop_propagation();
                    let tman = &mut tab_manager.write();
                    tman.remove_tab(item.id.clone());
                },
                Icon {
                    icon: ld_icons::LdX,
                    width: 10,
                    height: 10,
                    class: "invisible group-hover:visible"
                }
            }
        }
    };
}
