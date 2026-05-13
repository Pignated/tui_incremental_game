use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, HorizontalAlignment, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::{
    event::{AppEvent, EventHandler},
    generator::{GeneratorRefCellWrapper, generator_list::GeneratorList},
    resources::ResourceManager,
};

pub struct App<'a> {
    widget: AppWidget<'a>,
    state: ListState,
}

impl<'a> App<'a> {
    pub fn new(widget: AppWidget<'a>, mut state: ListState) -> Self {
        state.select(Some(0));
        Self { widget, state }
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
    pub resource_manager: ResourceManager<'a>,
    generators: Vec<GeneratorRefCellWrapper<'a>>,
    generator_list: GeneratorList<'a>,
}

impl<'a> AppWidget<'a> {
    pub fn new() -> Self {
        let resource_manager = ResourceManager::new();
        let generator_list: GeneratorList<'_> = GeneratorList::default();
        let mut generators = Vec::new();
        let (a, b) = generator_list.get_initials();
        generators.push(a);
        generators.push(b);

        Self {
            running: true,
            events: EventHandler::new(),
            resource_manager,
            generators,
            generator_list,
        }
    }

    fn handle_events(&mut self, state: &mut ListState) -> color_eyre::Result<()> {
        match self.events.next()? {
            crate::event::Event::Tick => {
                self.resource_manager.tick();
                for gener in &mut self.generators {
                    self.resource_manager.change(gener.borrow_mut().tick());
                }
            }
            crate::event::Event::Crossterm(event) => {
                if let event::Event::Key(key_event) = event {
                    match key_event.code {
                        KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                        KeyCode::Enter | KeyCode::Char(' ') => self.events.send(AppEvent::Select),
                        KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::GoUp),
                        KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::GoDown),
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
                crate::event::AppEvent::GoLeft | crate::event::AppEvent::GoRight => {}
                crate::event::AppEvent::Select => {
                    let gen_idx = state.selected.unwrap();
                    let mut can_afford = true;
                    let gener_cost = self.generators[gen_idx].borrow().get_cost().clone();
                    let res_arr = self.resource_manager.get_resources_arr();
                    for (i, v) in res_arr.iter().enumerate() {
                        if gener_cost[i] > *v {
                            can_afford = false;
                            break;
                        }
                    }
                    if can_afford {
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
                        for rc in res_change {
                            self.resource_manager.change(rc);
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
    type State = ListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut ListState,
    ) where
        Self: Sized,
    {
        let vert = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [title_area, main_area, status_area] = vert.areas(area);
        let hor = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(5)]);
        let [left_area, right_area] = hor.areas(main_area);
        let top_block = Block::new()
            .borders(Borders::TOP)
            .title("Tuincremental")
            .title_alignment(HorizontalAlignment::Center)
            .border_set(symbols::border::DOUBLE);
        top_block.render(title_area, buf);
        let resources_block = Block::bordered()
            .title("Resources")
            .title_alignment(HorizontalAlignment::Center);
        let resources_paragraph =
            Paragraph::new(self.resource_manager.resource_lines.clone()).block(resources_block);
        resources_paragraph.render(left_area, buf);
        let generator_block = Block::bordered()
            .title("Generators")
            .title_alignment(HorizontalAlignment::Center);
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
        let bottom_block = Block::new().title("By Pignated").title_style(
            Style::new()
                .bg(Color::Rgb(250, 201, 5))
                .fg(Color::Rgb(250, 58, 5)),
        );
        bottom_block.render(status_area, buf);
        StatefulWidget::render(generator_list, right_area, buf, state)
    }
}
