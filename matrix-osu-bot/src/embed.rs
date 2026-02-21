use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use maud::{html, Markup};

pub(crate) struct EmbedBuilder {
    title: String,
    fields: Vec<(String, String)>,
    url: Option<String>,
    url_text: Option<String>,
}

impl EmbedBuilder {
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            fields: Vec::new(),
            url: None,
            url_text: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();

        self
    }

    pub fn field(mut self, field_name: impl Into<String>, field_value: impl Into<String>) -> Self {
        self.fields.push((field_name.into(), field_value.into()));

        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());

        self
    }

    pub fn url_text(mut self, url_text: impl Into<String>) -> Self {
        self.url_text = Some(url_text.into());

        self
    }

    pub fn build(&self) -> RoomMessageEventContent {
        RoomMessageEventContent::notice_html(self.to_plain(), self.to_html())
    }

    pub fn to_plain(&self) -> String {
        let fields = self.fields.iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join(" | ");

        format!("{} — {}", self.title, fields)
    }

    fn to_html(&self) -> Markup {
        let url_text = match &self.url_text {
            Some(string) => string,
            None => "View URL",
        };

        html! {
            blockquote {
                  strong { (self.title) } br;

                  @for (i, (name, value)) in self.fields.iter().enumerate() {
                      strong { (name) ":" } " " (value)
                      @if i < self.fields.len() - 1 { br; }
                  }

                  @if let Some(url) = &self.url {
                      br;
                      a href=(url) { (url_text) }
                  }
              }
        }
    }
}