
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::{cmark_resume, State};
use sapiens::chains::Message;
use sapiens::context::{ChatEntry, ChatEntryFormatter, MessageFormatter};
use sapiens::models::Role;

/// Chat entry formatter that renders the chat entry in markdown
pub struct Formatter {}

impl ChatEntryFormatter for Formatter {
    fn format(&self, entry: &ChatEntry) -> String {
        let msg = entry.msg.clone();
        match entry.role {
            Role::User => format!(":earth_americas:\n{}", msg),
            Role::Assistant => format!(":robot:\n{}", msg),
            Role::System => format!(":rooster:\n{}", msg),
            Role::Function => format!(":gear:\n{}", msg),
            Role::Tool => format!(":wrench:\n{}", msg),
        }
    }
}

impl MessageFormatter for Formatter {
    fn format(&self, msg: &Message) -> String {
        msg.to_string()
    }
}

/// Size in characters of an event once rendered in markdown
fn md_event_size(event: &Event) -> usize {
    match event {
        Event::Text(text) => text.len(),
        Event::Code(text) => text.len(),
        Event::Html(text) => text.len(),
        Event::FootnoteReference(text) => text.len(),
        Event::SoftBreak => 1,
        Event::HardBreak => 2,
        Event::Rule => 1,
        Event::TaskListMarker(_) => 3,
        Event::Start(tag) => {
            let len = match tag {
                Tag::Paragraph => 2,
                Tag::Heading(level, id, classes) => {
                    *level as usize
                        + 1
                        + id.map(|x| x.len()).unwrap_or_default()
                        + classes.iter().map(|x| x.len()).sum::<usize>()
                }
                Tag::BlockQuote => 2,
                Tag::CodeBlock(CodeBlockKind::Indented) => 2,
                Tag::CodeBlock(CodeBlockKind::Fenced(fence)) => fence.len() + 3,
                Tag::List(_) => 2,
                Tag::Item => 2,
                Tag::FootnoteDefinition(d) => d.len() + 3,
                Tag::Table(_) => 4,
                Tag::TableHead => 2,
                Tag::TableRow => 3,
                Tag::TableCell => 3,
                Tag::Emphasis => 2,
                Tag::Strong => 2,
                Tag::Strikethrough => 2,
                Tag::Link(_, u, t) => 4 + u.len() + t.len(),
                Tag::Image(_, u, t) => 4 + u.len() + t.len(),
            };
            len
        }
        Event::End(_tag) => 4, // random
    }
}

/// Sanitize messages for Discord
///
/// Tries to split mardown messages into multiple messages if they are too
/// long for Discord.
///
/// If any split is still too long, truncate it.
pub(crate) fn sanitize_msgs_for_discord(msgs: Vec<String>) -> Vec<String> {
    msgs.into_iter()
        .flat_map(|m| split_msgs(m, 1800))
        .map(|mut x| {
            if x.len() > 1800 - 3 {
                x.truncate(1800);
                x.push_str("...");
            }
            x
        })
        .collect()
}

fn is_block_delimiter(t: &Tag) -> bool {
    match t {
        Tag::Paragraph => true,
        Tag::Heading(_, _, _) => true,
        Tag::BlockQuote => true,
        Tag::CodeBlock(_) => true,
        Tag::List(_) => true,
        Tag::Item => false,