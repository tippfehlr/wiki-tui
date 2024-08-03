use clap::Parser;

use crate::action::{Action, ActionPacket, SearchAction};
use wiki_api::languages::Language;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Search for an article
    #[arg(value_name = "QUERY")]
    search_query: Option<String>,

    #[arg(value_name = "LANGUAGE", short = 'l', long = "language")]
    language: Option<String>,

    #[cfg(debug_assertions)]
    #[arg(value_name = "PATH", long = "page")]
    debug_page: Option<std::path::PathBuf>,
}

pub fn match_cli() -> Option<ActionPacket> {
    let cli = Cli::parse();

    let mut packet = ActionPacket::default();

    if let Some(search_query) = cli.search_query {
        packet.add_action(Action::ExitSearchBar);
        packet.add_action(Action::SwitchContextSearch);
        packet.add_action(Action::Search(SearchAction::StartSearch(search_query)));
    }

    if let Some(language) = cli.language {
        let language = Language::from(language);
        packet.add_action(Action::Search(SearchAction::ChangeLanguage(language)));
    }

    #[cfg(debug_assertions)]
    if let Some(ref debug_page) = cli.debug_page {
        if let Some(page) = wiki_api::page::Page::from_path(debug_page) {
            packet.add_action(Action::SwitchContextPage);
            packet.add_action(Action::PageViewer(
                crate::action::PageViewerAction::DisplayPage(page),
            ));
        }
    }

    Some(packet)
}
