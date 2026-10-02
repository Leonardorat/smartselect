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
        let event = match event {
    Ok(event) => event,
    Err(error) => {
        eprintln!("Ignoring accessibility event error: {error}");
        continue;
    }
};

        let Event::Object(
            ObjectEvents::TextSelectionChanged(event)
        ) = event
        else {
            continue;
        };

        let object = event.item;
        //
        match adapter.should_ignore(&object).await {
    Ok(false) => {}
    Ok(true) => continue,
    Err(error) => {
        // Skip applications whose identity cannot be checked.
        eprintln!("Skipping unidentified application: {error}");
        continue;
    }
}
//
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