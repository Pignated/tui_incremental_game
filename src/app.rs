use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, HorizontalAlignment, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::{
    event::{AppEvent, EventHandler},
    generator::{GeneratorRefCellWrapper, generator_list::GeneratorList},
    resources::{RESOURCE_COUNT, ResManager, resource_change::ResourceChange},
    upgrades::{Upgrade, upgrade_manager::UpgradeManager},
};

pub struct App<'a> {
    widget: AppWidget<'a>,
    state: AppState,
}

impl<'a> App<'a> {
    pub fn new(widget: AppWidget<'a>) -> Self {
        Self {
            widget,
            state: AppState::new(),
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.widget.running {
            terminal.draw(|frame| {
                frame.render_stateful_widget(&self.widget, frame.area(), &mut self.state)
            })?;
            self.widget.handle_events(&mut self.state)?
        }
        Ok(())
    }
}

pub struct AppWidget<'a> {
    pub events: EventHandler,
    pub running: bool,
    pub resource_manager: ResManager<'a>,
    generators: Vec<GeneratorRefCellWrapper<'a>>,
    generator_list: GeneratorList<'a>,
    purchasable_upgrades: Vec<Upgrade<'a>>,
    upgrade_manager: UpgradeManager<'a>,
}

impl<'a> AppWidget<'a> {
    pub fn new() -> Self {
        let resource_manager = ResManager::new();
        let mut generator_list: GeneratorList<'_> = GeneratorList::default();
        let mut generators = Vec::new();
        generators.push(generator_list.get_next().unwrap());
        generators.push(generator_list.get_next().unwrap());
        let purchasable_upgrades = Vec::new();
        let upgrade_manager = UpgradeManager::new(&generator_list);
        Self {
            running: true,
            events: EventHandler::new(),
            resource_manager,
            generators,
            generator_list,
            purchasable_upgrades,
            upgrade_manager,
        }
    }

    fn handle_events(&mut self, state: &mut AppState) -> color_eyre::Result<()> {
        match self.events.next()? {
            crate::event::Event::Tick => {
                self.resource_manager.tick();
                for gener in &mut self.generators {
                    self.resource_manager.change(gener.borrow_mut().tick());
                }
                self.upgrade_manager
                    .poll_requirement_reached(self.resource_manager.get_all_total_counts());
                self.purchasable_upgrades
                    .append(&mut self.upgrade_manager.ready_upgrades.clone());
                self.upgrade_manager.ready_upgrades.clear();
            }
            crate::event::Event::Crossterm(event) => {
                if let event::Event::Key(key_event) = event {
                    match key_event.code {
                        KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                        KeyCode::Enter | KeyCode::Char(' ') => self.events.send(AppEvent::Select),
                        KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::GoUp),
                        KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::GoDown),
                        KeyCode::Left | KeyCode::Char('h') => self.events.send(AppEvent::GoLeft),
                        KeyCode::Right | KeyCode::Char('l') => self.events.send(AppEvent::GoRight),
                        _ => {}
                    }
                    if key_event.code == KeyCode::Char('q') {
                        self.events.send(crate::event::AppEvent::Quit);
                    }
                }
            }
            crate::event::Event::App(app_event) => match app_event {
                crate::event::AppEvent::GoUp => {
                    state.previous();
                }
                crate::event::AppEvent::GoDown => {
                    state.next();
                }
                crate::event::AppEvent::GoLeft | crate::event::AppEvent::GoRight => {
                    state.switch();
                }
                crate::event::AppEvent::Select => {
                    if !state.selected_upgrade {
                        let gen_idx = state.generator_state.selected.unwrap();
                        let gener_cost = self.generators[gen_idx].borrow().get_cost().clone();

                        if self.resource_manager.can_afford(gener_cost) {
                            let first_bought = self.generators[gen_idx].borrow().get_count() == 0;
                            if first_bought {
                                match self.generator_list.get_next() {
                                    Some(gener) => {
                                        self.generators.push(gener);
                                    }
                                    _ => (),
                                }
                            }
                            let res_change = self.generators[gen_idx].borrow_mut().buy_next();
                            self.resource_manager.change(res_change);
                        }
                    } else {
                        if let Some(upgrade_idx) = state.upgrade_state.selected {
                            {
                                if let Some(upgrade) = self.purchasable_upgrades.get(upgrade_idx) {
                                    if self.resource_manager.can_afford(upgrade.cost) {
                                        self.resource_manager
                                            .change(ResourceChange::Decrease { val: upgrade.cost });
                                        self.generator_list.apply_upgrade(upgrade.clone());
                                        self.purchasable_upgrades.remove(upgrade_idx);
                                    }
                                }
                            }
                            self.upgrade_manager.poll_requirement_reached(
                                self.resource_manager.get_all_total_counts(),
                            );
                        }
                    }
                }

                crate::event::AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }
    fn quit(&mut self) {
        self.running = false;
    }
}

impl<'a> StatefulWidget for &AppWidget<'a> {
    type State = AppState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut AppState,
    ) where
        Self: Sized,
    {
        // Space Allocation
        let vert = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [title_area, main_area, status_area] = vert.areas(area);
        let hor = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)]);
        let [left_area, right_area] = hor.areas(main_area);
        let left_layout = Layout::vertical([
            Constraint::Length((RESOURCE_COUNT + 2) as u16),
            Constraint::Fill(1),
        ]);
        let [resource_area, upgrades_area] = left_layout.areas(left_area);
        //Rendering Header
        let top_block = Block::new()
            .borders(Borders::TOP)
            .title("Tuincremental")
            .title_alignment(HorizontalAlignment::Center)
            .border_set(symbols::border::DOUBLE);
        top_block.render(title_area, buf);

        //Rendering Resource list
        let resources_block = Block::bordered()
            .title("Resources")
            .title_alignment(HorizontalAlignment::Center);
        let resources_paragraph =
            Paragraph::new(Vec::from(self.resource_manager.resource_lines.clone()))
                .block(resources_block);
        resources_paragraph.render(resource_area, buf);

        //Rendering Generator list
        let generator_block;
        if state.selected_upgrade {
            generator_block = Block::bordered()
                .title("Generators")
                .title_alignment(HorizontalAlignment::Center);
        } else {
            generator_block = Block::bordered()
                .title("Generators")
                .title_alignment(HorizontalAlignment::Center)
                .style(Style::new().fg(Color::Yellow));
        }
        let builder = ListBuilder::new(|context| {
            let mut item = self.generators[context.index].clone();
            let mut size = 3;
            item = if context.is_selected {
                size += 2;
                item.borrow_mut().block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().yellow()),
                );
                item
            } else {
                item.borrow_mut().clear_block();
                item
            };
            (item.clone(), size)
        });
        let generator_list = ListView::new(builder, self.generators.len()).block(generator_block);
        StatefulWidget::render(generator_list, right_area, buf, &mut state.generator_state);

        let upgrade_width = upgrades_area.width;
        let upgrade_block = if state.selected_upgrade {
            Block::bordered()
                .title("Upgrades")
                .title_alignment(HorizontalAlignment::Center)
                .style(Style::default().yellow())
        } else {
            Block::bordered()
                .title("Upgrades")
                .title_alignment(HorizontalAlignment::Center)
        };
        let upgrade_count = self.purchasable_upgrades.len();
        let upgrade_builder = ListBuilder::new(|context| {
            let mut item = self.purchasable_upgrades[context.index].clone();
            let para = Paragraph::new(item.description.clone()).wrap(Wrap { trim: true });
            let mut size = para.line_count(upgrade_width) + 3;
            item = if context.is_selected {
                size += 2;
                item.block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().yellow()),
                );
                item
            } else if context.index + 1 < upgrade_count {
                item.block(Block::default().borders(Borders::BOTTOM));
                size += 1;
                item
            } else {
                item.clear_block();
                item
            };
            (item, size as u16)
        });
        let upgrade_list =
            ListView::new(upgrade_builder, self.purchasable_upgrades.len()).block(upgrade_block);
        StatefulWidget::render(upgrade_list, upgrades_area, buf, &mut state.upgrade_state);
        //Rendering Footer
        let mut bottom_block = Block::new().title("By Pignated");
        bottom_block = bottom_block.title_style(
            Style::new()
                .bg(Color::Rgb(250, 201, 5))
                .fg(Color::Rgb(250, 58, 5)),
        );
        bottom_block.render(status_area, buf);
    }
}
pub struct AppState {
    upgrade_state: ListState,
    generator_state: ListState,
    selected_upgrade: bool,
    upgrade_last_selected: Option<usize>,
    generator_last_selected: Option<usize>,
}

impl AppState {
    fn new() -> Self {
        let mut upgrade_state = ListState::default();
        let mut generator_state = ListState::default();
        upgrade_state.select(None);
        generator_state.select(Some(0));
        AppState {
            upgrade_state,
            generator_state,
            selected_upgrade: false,
            upgrade_last_selected: Some(0),
            generator_last_selected: Some(0),
        }
    }
    fn previous(&mut self) {
        if self.selected_upgrade {
            self.upgrade_state.previous();
        } else {
            self.generator_state.previous();
        }
    }
    fn next(&mut self) {
        if self.selected_upgrade {
            self.upgrade_state.next();
        } else {
            self.generator_state.next();
        }
    }
    fn switch(&mut self) {
        if self.selected_upgrade {
            self.selected_upgrade = false;
            self.upgrade_last_selected = self.upgrade_state.selected;
            self.upgrade_state.select(None);
            self.generator_state.select(self.generator_last_selected);
        } else {
            self.selected_upgrade = true;
            self.generator_last_selected = self.generator_state.selected;
            self.generator_state.select(None);
            self.upgrade_state.select(self.upgrade_last_selected);
        }
    }
}
