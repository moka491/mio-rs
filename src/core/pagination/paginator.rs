use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{channel::Message, id::MessageId},
};

pub struct Paginator {
    paginations: Vec<Pagination>,
}

impl Paginator {
    pub fn handle_reaction(msg_id: MessageId) {}

    pub fn add_pagination(msg: &Message) {}

    pub fn remove_pagination(msg: &Message) {}
}

pub struct Pagination {
    message: Message,
    pages: Vec<String>,
    current_page: usize,
    total_page_count: usize,
    page_builder: fn(String) -> CreateEmbed,
    fetch_next_pages: fn() -> Vec<String>,
}

impl Pagination {
    pub fn change_page(&mut self, ctx: Context, which_page: PageChange) {
        let new_page_num = match which_page {
            PageChangeRequest::First => 1,
            PageChangeRequest::Previous => self.current_page - 1,
            PageChangeRequest::Next => self.current_page + 1,
            PageChangeRequest::Last => self.pages.len(),
        };

        let next_page_data = match self.pages.get(new_page_num - 1) {
            Some(data) => data,
            None => return,
        };

        let _ = self
            .message
            .edit(ctx.http, |m| m.embed(|mut e| e.0 = next_embed.0.clone()));

        self.current_page = new_page_num;
    }
}

pub enum PageChange {
    First,
    Previous,
    Next,
    Last,
}
