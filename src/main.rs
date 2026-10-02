mod adapters;
mod core;

use anyhow::Result;

use futures_util::StreamExt;

use atspi::{
    Event,
    ObjectEvents,
};

use atspi::events::object::{
    TextSelectionChangedEvent,
};

use adapters::atspi::AtspiAdapter;
use core::SelectionEngine;

#[tokio::main]
async fn main() -> Result<()> {
    let adapter =
        AtspiAdapter::connect().await?;

    let engine =
        SelectionEngine::new();

    adapter
        .connection()
        .register_event::<
            TextSelectionChangedEvent
        >()
        .await?;

    let events =
        adapter.connection().event_stream();

    futures_util::pin_mut!(events);

    println!("SmartSelect running");

    while let Some(event) =
        events.next().await
    {
        let event = event?;

        let Event::Object(
            ObjectEvents::TextSelectionChanged(event)
        ) = event
        else {
            continue;
        };

        let object = event.item;

        let context =
            match adapter
                .read_context(&object)
                .await
            {
                Ok(context) => context,

                Err(error) => {
                    eprintln!(
                        "Could not read text object: {error}"
                    );

                    continue;
                }
            };

        let Some(range) =
            engine.select(&context)
        else {
            continue;
        };

        if let Err(error) =
            adapter
                .apply_selection(
                    &object,
                    range,
                )
                .await
        {
            eprintln!(
                "Could not apply selection: {error}"
            );
        }
    }

    Ok(())
}