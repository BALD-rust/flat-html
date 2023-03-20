// TODO: use a macro to generate parse definitions instead of dumb python script and comments

use std::str::FromStr;

use anyhow::Result;

// We can potentially store attributes inside this enum
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TagKind {
    Html, // <html>
    Meta, // <meta>
    Title, // <title>
    Script, // <script>
    Head, // <head>
    Body, // <body>
    Div, // <div>
    Span, // <span>
    Input, // <input>
    Label, // <label>
    Table, // <table>
    UnorderedList, // <ul>
    ListItem, // <li>
    Style, // <style>
    Bold, // <b>
    Italic, // <i>
    Heading(u32), // <h{level}> .. ?
    Link, // <a>
    Paragraph, // <p>
    Code, // <code>
    LineBreak, // <br>
    Unknown,
    // TODO:
    // <dt>, <dd>, <sup>, <pre>
    // <th>, <tr>, <tb>, <td>
    // <math> ?
}

#[derive(Debug, Clone)]
pub enum Element {
    Text(String),
    Tag(TagKind),
    EndTag(TagKind),
    LineBreak,
    IgnoreTag // Setting the element to this will ignore the tag, but parse the children
}

#[derive(Debug)]
pub struct FlatHtml(pub Vec<Element>);

impl FromStr for TagKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "html" => TagKind::Html,
            "meta" => TagKind::Meta,
            "title" => TagKind::Title,
            "script" => TagKind::Script,
            "head" => TagKind::Head,
            "body" => TagKind::Body,
            "div" => TagKind::Div,
            "span" => TagKind::Span,
            "input" => TagKind::Input,
            "label" => TagKind::Label,
            "table" => TagKind::Table,
            "ul" => TagKind::UnorderedList,
            "li" => TagKind::ListItem,
            "style" => TagKind::Style,
            "b" => TagKind::Bold,
            "i" => TagKind::Italic,
            "h1" => TagKind::Heading(1),
            "a" => TagKind::Link,
            "p" => TagKind::Paragraph,
            "code" => TagKind::Code,
            "br" => TagKind::LineBreak,
            _ => TagKind::Unknown
        })
    }
}

pub fn to_html_tag(kind: &TagKind) -> String {
    match kind {
        TagKind::Html => String::from("html"),
        TagKind::Meta => String::from("meta"),
        TagKind::Title => String::from("title"),
        TagKind::Script => String::from("script"),
        TagKind::Head => String::from("head"),
        TagKind::Body => String::from("body"),
        TagKind::Div => String::from("div"),
        TagKind::Span => String::from("span"),
        TagKind::Input => String::from("input"),
        TagKind::Label => String::from("label"),
        TagKind::Table => String::from("table"),
        TagKind::UnorderedList => String::from("ul"),
        TagKind::ListItem => String::from("li"),
        TagKind::Style => String::from("style"),
        TagKind::Bold => String::from("b"),
        TagKind::Italic => String::from("i"),
        TagKind::Heading(u32) => String::from("h1"),
        TagKind::Link => String::from("a"),
        TagKind::Paragraph => String::from("p"),
        TagKind::Code => String::from("code"),
        TagKind::LineBreak => String::from("br"),
        TagKind::Unknown => String::from("div") // doesnt do anything, so this is probably a "decent" default, until we error on it.
    }
}