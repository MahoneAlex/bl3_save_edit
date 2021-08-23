use iced::{button, svg, Column, Container, Length, Row};
use strum::Display;

use crate::bl3_ui::{InteractionMessage, Message};
use crate::resources::svgs::{CHARACTER, CURRENCY, GENERAL, INVENTORY};
use crate::views::manage_save::character::CharacterState;
use crate::views::manage_save::currency::CurrencyState;
use crate::views::manage_save::general::GeneralState;
use crate::views::manage_save::inventory::InventoryState;
use crate::views::manage_save::{
    character, currency, general, inventory, ManageSaveInteractionMessage, ManageSaveState,
};
use crate::views::{tab_bar_button, ManageTabBarStyle};

#[derive(Debug, Default)]
pub struct SaveViewState {
    tab_bar_state: SaveTabBarState,
    pub general_state: GeneralState,
    pub character_state: CharacterState,
    pub currency_state: CurrencyState,
    pub inventory_state: InventoryState,
}

#[derive(Debug, Default)]
pub struct SaveTabBarState {
    general_button_state: button::State,
    character_button_state: button::State,
    inventory_button_state: button::State,
    currency_button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum SaveTabBarInteractionMessage {
    General,
    Character,
    Inventory,
    Currency,
}

#[derive(Debug, Display, PartialEq)]
#[strum(serialize_all = "title_case")]
pub enum SaveTabBarView {
    General,
    Character,
    Inventory,
    Currency,
}

pub fn view<'a>(
    manage_save_state: &'a mut ManageSaveState,
    tab_bar_view: &SaveTabBarView,
) -> Container<'a, Message> {
    let general_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .general_button_state,
        SaveTabBarView::General,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::General,
        )),
        svg::Handle::from_memory(GENERAL),
        100,
    );

    let character_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .character_button_state,
        SaveTabBarView::Character,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Character,
        )),
        svg::Handle::from_memory(CHARACTER),
        115,
    );

    let inventory_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .inventory_button_state,
        SaveTabBarView::Inventory,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Inventory,
        )),
        svg::Handle::from_memory(INVENTORY),
        115,
    );

    let currency_button = tab_bar_button(
        &mut manage_save_state
            .save_view_state
            .tab_bar_state
            .currency_button_state,
        SaveTabBarView::Currency,
        tab_bar_view,
        InteractionMessage::ManageSaveInteraction(ManageSaveInteractionMessage::TabBar(
            SaveTabBarInteractionMessage::Currency,
        )),
        svg::Handle::from_memory(CURRENCY),
        105,
    );

    let tab_bar = Container::new(
        Row::new()
            .push(general_button)
            .push(character_button)
            .push(inventory_button)
            .push(currency_button),
    )
    .width(Length::Fill)
    .style(ManageTabBarStyle);

    let tab_content = match tab_bar_view {
        SaveTabBarView::General => {
            general::view(&mut manage_save_state.save_view_state.general_state)
        }
        SaveTabBarView::Character => {
            character::view(&mut manage_save_state.save_view_state.character_state)
        }
        SaveTabBarView::Currency => {
            currency::view(&mut manage_save_state.save_view_state.currency_state)
        }
        SaveTabBarView::Inventory => {
            inventory::view(&mut manage_save_state.save_view_state.inventory_state)
        }
    };

    let all_contents = Column::new().push(tab_bar).push(tab_content);

    Container::new(all_contents)
        .width(Length::Fill)
        .height(Length::Fill)
}
