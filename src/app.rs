use crossterm::event::{self, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, HorizontalAlignment, Layout},
    style::{Color, Style},
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::{
    event::{AppEvent, EventHandler},
    generator::{Generator, generator_list::GeneratorList},
    resources::{Resource, ResourceChange, ResourceType},
};

pub struct App<'a> {
    widget: AppWidget<'a>,
    state: ListState,
}

impl<'a> App<'a> {
    pub fn new(widget: AppWidget<'a>, state: ListState) -> Self {
        Self { widget, state }
    }
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.widget.running {
            terminal.draw(|frame| {
                frame.render_stateful_widget(&self.widget, frame.area(), &mut self.state)
            })?;
            self.widget.handle_events()?
        }
        Ok(())
    }
}

pub struct AppWidget<'a> {
    pub events: EventHandler,
    pub running: bool,
    pub resources: Vec<Resource>,
    generator_selected: u64,
    generators: Vec<Generator<'a>>,
    generator_count: u64,
    debug: String,
    generator_list: GeneratorList<'a>,
}

impl<'a> AppWidget<'a> {
    pub fn new() -> Self {
        let mut resources = Vec::new();
        resources.push(Resource::new(
            "Wood".to_owned(),
            ResourceType::WOOD.get_color(),
            ResourceType::WOOD,
        ));
        let generator_list: GeneratorList<'_> = GeneratorList::default();
        let mut generators = Vec::new();
        let (a, b) = generator_list.get_initials();
        generators.push(a);
        generators.push(b);

        Self {
            running: true,
            events: EventHandler::new(),
            resources,
            generator_selected: 0,
            generator_count: 2,
            generators,
            debug: "Init".to_owned(),
            generator_list,
        }
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            crate::event::Event::Tick => {
                for res in &mut self.resources {
                    res.tick();
                }
                for gener in &mut self.generators {
                    match gener.tick() {
                        ResourceChange::Increase { amts, .. } => {
                            for x in amts {
                                if let Some(resource) =
                                    self.resources.iter_mut().find(|y| y.resource_type == x.0)
                                {
                                    resource.increase(x.1);
                                } else {
                                    let mut new_resource = Resource::new_from_type(x.0);
                                    new_resource.increase(x.1);
                                    self.resources.push(new_resource);
                                }
                            }
                        }
                        ResourceChange::Decrease { amts, .. } => {
                            for x in amts {
                                if let Some(resource) =
                                    self.resources.iter_mut().find(|y| y.resource_type == x.0)
                                {
                                    resource.decrease(x.1);
                                } else {
                                    let mut new_resource = Resource::new_from_type(x.0);
                                    new_resource.decrease(x.1);
                                    self.resources.push(new_resource);
                                }
                            }
                        }
                        ResourceChange::None => {}
                        ResourceChange::SingleIncrease { amt, resource_type } => {
                            if let Some(resource) = self
                                .resources
                                .iter_mut()
                                .find(|x| x.resource_type == resource_type)
                            {
                                resource.increase(amt);
                            } else {
                                let mut new_resource = Resource::new_from_type(resource_type);
                                new_resource.increase(amt);
                                self.resources.push(new_resource);
                            }
                        }
                    }
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
                    self.generator_selected = self
                        .generator_selected
                        .wrapping_sub(1)
                        .rem_euclid(self.generator_count);
                }
                crate::event::AppEvent::GoDown => {
                    self.generator_selected = (self.generator_selected + 1) % self.generator_count
                }
                crate::event::AppEvent::GoLeft | crate::event::AppEvent::GoRight => {}
                crate::event::AppEvent::Select => {
                    let gen_idx = self.generator_selected as usize;
                    let mut can_afford = true;
                    {
                        let generator = &self.generators[gen_idx];
                        let cost = generator.get_cost();
                        for (res, amt) in &cost {
                            if self.get_resource(*res).count < *amt {
                                can_afford = false;
                                break;
                            }
                        }
                    }
                    if can_afford {
                        if self.generators[gen_idx].get_count() == 0 {
                            match self.generator_list.get_next() {
                                Some(gener) => {
                                    self.generators.push(gener);
                                    self.generator_count += 1;
                                }
                                _ => (),
                            }
                        }
                        let res_change = self.generators[gen_idx].buy_next();
                        for res in &mut self.resources {
                            res.change(&res_change);
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
    fn get_resource(&mut self, res: ResourceType) -> &mut Resource {
        let idx = self.resources.iter().position(|x| x.resource_type == res);
        if let Some(i) = idx {
            &mut self.resources[i]
        } else {
            let new_res = Resource::new_from_type(res);
            self.resources.push(new_res);
            self.resources.last_mut().unwrap()
        }
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
        state.select(Some(self.generator_selected as usize));
        let resources_block = Block::bordered()
            .title("Resources")
            .title_alignment(HorizontalAlignment::Center);

        let mut resources_text: Vec<Line> = Vec::new();
        for res in &self.resources {
            resources_text.push(res.get_str());
        }

        let resources_paragraph = Paragraph::new(resources_text).block(resources_block);
        resources_paragraph.render(left_area, buf);
        let generator_block = Block::bordered()
            .title("Generators")
            .title_alignment(HorizontalAlignment::Center);
        let builder = ListBuilder::new(|context| {
            let mut item = self.generators[context.index].clone();
            let mut size = 3;
            item = if context.is_selected {
                size += 2;
                item.block(
                    Block::default()
                        .borders(Borders::TOP | Borders::BOTTOM)
                        .style(Style::default().yellow()),
                )
            } else {
                item.clear_block()
            };
            (item, size)
        });
        let generator_list = ListView::new(builder, self.generators.len()).block(generator_block);
        let bottom_block = Block::new()
            .title(format!("By pignated{0}", self.debug))
            .title_style(
                Style::new()
                    .bg(Color::Rgb(250, 201, 5))
                    .fg(Color::Rgb(250, 58, 5)),
            );
        bottom_block.render(status_area, buf);
        StatefulWidget::render(generator_list, right_area, buf, state)
    }
}
