use matrix_sdk::Client;
use crate::commands::CommandDataBuilder;

pub(crate) struct ContextContainer<'a> {
    command_data_builder: CommandDataBuilder,
    client: &'a Client,
}

impl<'a> ContextContainer<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            command_data_builder: CommandDataBuilder::default(),
            client
        }
    }

    pub fn add<T>(mut self, ctx: T) -> Self
    where
        T: Clone + Send + Sync + 'static
    {
        self.command_data_builder.add(ctx.clone());
        self.client.add_event_handler_context(ctx);

        self
    }

    pub fn finalise(self) {
        let command_data = self.command_data_builder.build();
        self.client.add_event_handler_context(command_data);
    }
}