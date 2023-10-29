use std::collections::HashMap;

use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, ScrollbarState},
};
use tokio::sync::mpsc;
use tracing::{debug, error};
use wiki_api::{
    languages::Language,
    page::{Page, PageRequest},
    Endpoint,
};

use crate::{
    action::{Action, PageAction},
    components::Component,
    renderer::{default_renderer::render_document, RenderedDocument},
    terminal::Frame,
    ui::centered_rect,
};

#[cfg(debug_assertions)]
use crate::renderer::test_renderer::{render_nodes_raw, render_tree_data, render_tree_raw};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Renderer {
    #[default]
    Default,

    #[cfg(debug_assertions)]
    TestRendererTreeData,
    #[cfg(debug_assertions)]
    TestRendererTreeRaw,
    #[cfg(debug_assertions)]
    TestRendererNodeRaw,
}

impl Renderer {
    pub fn next(&self) -> Self {
        match self {
            #[cfg(not(debug_assertions))]
            &Renderer::Default => Renderer::Default,

            #[cfg(debug_assertions)]
            &Renderer::Default => Renderer::TestRendererTreeData,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererTreeData => Renderer::TestRendererTreeRaw,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererTreeRaw => Renderer::TestRendererNodeRaw,
            #[cfg(debug_assertions)]
            &Renderer::TestRendererNodeRaw => Renderer::Default,
        }
    }
}

#[derive(Default)]
pub struct PageComponent {
    page: Option<Page>,
    renderer: Renderer,
    render_cache: HashMap<u16, RenderedDocument>,

    endpoint: Option<Endpoint>,
    language: Option<Language>,

    scroll_state: ScrollbarState,
    scroll: usize,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl PageComponent {
    fn build_page(&self, title: String) -> Result<PageRequest> {
        let endpoint = self
            .endpoint
            .clone()
            .ok_or(anyhow!("No Endpoint configured"))?;
        let language = self
            .language
            .clone()
            .ok_or(anyhow!("No Language configured"))?;

        Ok(Page::builder()
            .page(title)
            .endpoint(endpoint)
            .language(language))
    }

    fn open_page(&mut self, title: String) {
        self.page = None;
        self.flush_cache();

        let tx = self.action_tx.clone().unwrap();
        let page_request = match self.build_page(title) {
            Ok(page_request) => page_request,
            Err(error) => {
                error!("Unable to build the page request: {:?}", error);
                return;
            }
        };
        tokio::spawn(async move {
            tx.send(Action::EnterProcessing).unwrap();
            match page_request.fetch().await {
                Ok(page) => tx.send(Action::Page(PageAction::FinishPage(page))).unwrap(),
                Err(error) => error!("Unable to complete the fetch: {:?}", error),
            };
            tx.send(Action::ExitProcessing).unwrap();
        });
    }

    fn render_page(&mut self, width: u16) -> &RenderedDocument {
        if self.render_cache.get(&width).is_some() {
            return self.render_cache.get(&width).unwrap();
        }

        debug!(
            "rebuilding cache for renderer '{:?}' with width '{}'",
            self.renderer, width
        );
        let document = match self.renderer {
            Renderer::Default => render_document(&self.page.as_ref().unwrap().content, width),
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeData => {
                render_tree_data(&self.page.as_ref().unwrap().content)
            }
            #[cfg(debug_assertions)]
            Renderer::TestRendererTreeRaw => render_tree_raw(&self.page.as_ref().unwrap().content),
            #[cfg(debug_assertions)]
            Renderer::TestRendererNodeRaw => render_nodes_raw(&self.page.as_ref().unwrap().content),
        };

        self.render_cache.insert(width, document);
        self.render_cache.get(&width).unwrap()
    }

    fn switch_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
        self.flush_cache();
    }

    fn flush_cache(&mut self) {
        debug!("flushing '{}' cached renders", self.render_cache.len());
        self.render_cache.clear();
        self.scroll_state = ScrollbarState::default();
        self.scroll = 0;
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
        self.scroll_state = self.scroll_state.position(self.scroll as u16);
    }

    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
        self.scroll_state = self.scroll_state.position(self.scroll as u16);
    }
}

impl Component for PageComponent {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.action_tx = Some(sender);

        // FIXME: the endpoint and language should be set by the root component
        self.endpoint = Some(Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap());
        self.language = Some(Language::default());

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
                Action::Page(PageAction::SwitchRenderer(self.renderer.next()))
            }
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Page(page_action) => match page_action {
                PageAction::OpenPage(title) => self.open_page(title),
                PageAction::FinishPage(page) => self.page = Some(page),
                PageAction::SwitchRenderer(renderer) => self.switch_renderer(renderer),
            },
            Action::ScrollUp(amount) => self.scroll_up(amount),
            Action::ScrollDown(amount) => self.scroll_down(amount),
            Action::Resize(..) => self.flush_cache(),
            _ => (),
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        if self.page.is_none() {
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)),
                area,
            );
            f.render_widget(
                Paragraph::new("Processing").alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        let viewport_top = area.top().saturating_add(self.scroll as u16) as usize;
        let viewport_bottom = area.bottom().saturating_add(self.scroll as u16) as usize;

        let rendered_page = self.render_page(area.width);
        let lines: Vec<Line> = rendered_page
            .lines
            .iter()
            .enumerate()
            .filter(|(y, _)| &viewport_top <= y && y <= &viewport_bottom)
            .map(|(_, line)| {
                let mut spans: Vec<Span> = Vec::new();
                line.iter()
                    .map(|word| {
                        spans.push(Span::styled(
                            format!(
                                "{}{}",
                                word.content,
                                " ".repeat(word.whitespace_width as usize)
                            ),
                            word.style,
                        ));
                    })
                    .count();
                Line {
                    spans,
                    ..Default::default()
                }
            })
            .collect();

        f.render_widget(Paragraph::new(lines), area);
    }
}
