use core::fmt;
use std::{
    env,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, HorizontalAlignment, Layout},
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap},
};
use serde::{Deserialize, Serialize};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::{
    VERSION, XOR_MASK,
    event::EventHandler,
    generator::{
        GeneratorRefCellWrapper,
        generator_list::GeneratorList,
        generator_save::{GeneratorListSave, GeneratorSave},
    },
    resources::{RESOURCE_COUNT, ResManager, resource_change::ResourceChange},
    upgrades::{
        Upgrade,
        upgrade_manager::UpgradeManager,
        upgrade_save::{UpgradeListSave, UpgradeSave},
    },
};

pub struct App {
    widget: AppWidget,
    state: AppState,
    file_to_save: Option<String>,
}

impl App {
    pub fn new(widget: AppWidget, file_to_save: Option<String>) -> Self {
        Self {
            widget,
            state: AppState::new(),
            file_to_save,
        }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        terminal.draw(|frame| {
            frame.render_stateful_widget(&self.widget, frame.area(), &mut self.state)
        })?;
        while self.widget.running {
            self.widget.handle_events(&mut self.state)?;
            terminal.draw(|frame| {
                frame.render_stateful_widget(&self.widget, frame.area(), &mut self.state)
            })?;
        }
        let saved = AppSave::new(
            self.widget.generator_list,
            self.widget.generators,
            self.widget.resource_manager,
            self.widget.purchasable_upgrades,
            self.widget.upgrade_manager,
        );
        save_app(saved, self.file_to_save)?;
        Ok(())
    }
}
pub struct AppWidget {
    pub events: EventHandler,
    pub running: bool,
    pub resource_manager: ResManager,
    generators: Vec<GeneratorRefCellWrapper>,
    generator_list: GeneratorList,
    purchasable_upgrades: Vec<Upgrade>,
    upgrade_manager: UpgradeManager,
    bottom_text: String,
}

impl AppWidget {
    pub fn new() -> Self {
        let resource_manager = ResManager::new();
        let mut generator_list: GeneratorList = GeneratorList::default();
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
            bottom_text: String::from("By Pignated"),
        }
    }
    pub fn from_save(app_save: AppSave) -> Self {
        let mut a = Self {
            events: EventHandler::new(),
            running: true,
            resource_manager: app_save.resource_manager,
            generators: app_save
                .active_generators
                .iter()
                .enumerate()
                .map(|(i, s)| GeneratorRefCellWrapper::new(GeneratorSave::to_gen(s), i as u64))
                .collect(),
            generator_list: GeneratorList::from_save(app_save.generator_list),
            purchasable_upgrades: app_save
                .purchasable_upgrades
                .iter()
                .map(UpgradeSave::to_upgr)
                .collect(),
            upgrade_manager: UpgradeManager::from_save(app_save.upgrade_manager),
            bottom_text: String::from("By Pignated"),
        };
        for i in &a.generators {
            a.generator_list.add_in_app_gen(i.clone());
        }
        a
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
                        KeyCode::Char('q') => self.quit(),
                        KeyCode::Enter | KeyCode::Char(' ') => self.handle_select(state),
                        KeyCode::Up | KeyCode::Char('k') => state.previous(),
                        KeyCode::Down | KeyCode::Char('j') => state.next(),
                        KeyCode::Left
                        | KeyCode::Char('h')
                        | KeyCode::Right
                        | KeyCode::Char('l') => state.switch(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    fn handle_select(&mut self, state: &mut AppState) {
        if !state.selected_upgrade {
            let gen_idx = state.generator_state.selected.unwrap();
            let gener_cost = self.generators[gen_idx].borrow().get_cost().clone();

            if self.resource_manager.can_afford(gener_cost) {
                let first_bought = self.generators[gen_idx].borrow().get_count() == 0;
                if first_bought {
                    self.bottom_text =
                        String::from(self.generators[gen_idx].borrow().progress.to_string());
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
                            if upgrade_idx > 0 {
                                state.next();
                            }
                        }
                    }
                }
                self.upgrade_manager
                    .poll_requirement_reached(self.resource_manager.get_all_total_counts());
            }
        }
    }
    fn quit(&mut self) {
        self.running = false;
    }
}

pub fn load_saved_app(file_to_save: Option<String>) -> Result<AppSave, Box<dyn Error>> {
    let path;
    if let Some(file_str) = file_to_save {
        if file_str.starts_with("/") {
            path = PathBuf::new().join(file_str)
        } else {
            path = env::current_dir()?.join(file_str);
        }
    } else {
        path = env::current_exe()?
            .parent()
            .ok_or("FUCK ME MAN")?
            .join("game_save.bin");
    }
    let mut file = File::open(path.clone())?;
    println!("aaaaa");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    for (i, byte) in buffer.iter_mut().enumerate() {
        let bytemask = XOR_MASK[i % XOR_MASK.len()];
        *byte ^= bytemask;
    }

    let app_save: AppSave = match postcard::from_bytes(&buffer) {
        Ok(data) => data,
        Err(_) => {
            ratatui::restore();
            return Err(Box::new(FileError::FileAlreadyExists(
                path.to_str().expect("").to_string(),
            )));
        }
    };
    Ok(app_save)
}
pub fn save_app(app: AppSave, file_to_save: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut buffer = postcard::to_allocvec(&app)?;
    for (i, byte) in buffer.iter_mut().enumerate() {
        let bytemask = XOR_MASK[i % XOR_MASK.len()];
        *byte ^= bytemask;
    }
    let path;
    if let Some(file_str) = file_to_save {
        if file_str.starts_with("/") {
            path = PathBuf::new().join(file_str)
        } else {
            path = env::current_dir()?.join(file_str);
        }
    } else {
        path = env::current_exe()?
            .parent()
            .ok_or("FUCK ME MAN")?
            .join("game_save.bin");
    }
    let mut file = File::create(path)?;

    file.write_all(&mut buffer)?;
    Ok(())
}
impl StatefulWidget for &AppWidget {
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
        let resource_lines: Vec<Line<'_>> = self
            .resource_manager
            .resource_lines
            .iter()
            .map(|s| Line::from(String::from(s)))
            .collect();
        let resources_paragraph = Paragraph::new(resource_lines).block(resources_block);
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
                .border_style(Style::new().fg(Color::Cyan));
        }
        let builder = ListBuilder::new(|context| {
            let mut item = self.generators[context.index].clone();
            let size = 3;
            if context.is_selected {
                item.select();
            } else {
                item.deselect()
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
                .border_style(Style::new().fg(Color::Cyan))
        } else {
            Block::bordered()
                .title("Upgrades")
                .title_alignment(HorizontalAlignment::Center)
        };
        let upgrade_builder = ListBuilder::new(|context| {
            let mut item = self.purchasable_upgrades[context.index].clone();
            let para = Paragraph::new(item.description.clone()).wrap(Wrap { trim: true });
            let size = para.line_count(upgrade_width) + 4;
            if context.is_selected {
                item.select();
            } else {
                item.deselect();
            }

            (item, size as u16)
        });
        let upgrade_list =
            ListView::new(upgrade_builder, self.purchasable_upgrades.len()).block(upgrade_block);
        StatefulWidget::render(upgrade_list, upgrades_area, buf, &mut state.upgrade_state);
        //Rendering Footer
        let mut bottom_block = Block::new().title(self.bottom_text.clone());
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

#[derive(Serialize, Deserialize)]
pub struct AppSave {
    generator_list: GeneratorListSave,
    active_generators: Vec<GeneratorSave>,
    resource_manager: ResManager,
    purchasable_upgrades: Vec<UpgradeSave>,
    upgrade_manager: UpgradeListSave,
    resource_count: usize,
    version: String,
}

impl AppSave {
    pub fn new(
        generator_list: GeneratorList,
        active_generators: Vec<GeneratorRefCellWrapper>,
        resource_manager: ResManager,
        purchasable_upgrades: Vec<Upgrade>,
        upgrade_manager: UpgradeManager,
    ) -> Self {
        AppSave {
            generator_list: generator_list.to_save(),
            active_generators: active_generators
                .iter()
                .map(|g| g.borrow().to_save(true, g.idx))
                .collect(),
            resource_manager,
            purchasable_upgrades: purchasable_upgrades.iter().map(Upgrade::to_save).collect(),
            upgrade_manager: upgrade_manager.to_save(),
            resource_count: RESOURCE_COUNT,
            version: VERSION.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FileError {
    FileAlreadyExists(String),
}
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::FileAlreadyExists(str) => write!(
                f,
                "Error: File at '{str}' already exists and is not a valid save"
            ),
        }
    }
}
impl Error for FileError {}
