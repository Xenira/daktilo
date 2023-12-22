//! A library for turning your keyboard into a typewriter! 📇
//!
//! See [`daktilo`](https://github.com/orhun/daktilo).

#![warn(missing_docs)]

/// Error implementation.
pub mod error;

/// Logger.
pub mod logger;

/// File embedder.
pub mod embed;

/// Application state.
pub mod app;

/// Configuration file.
pub mod config;

/// Rodio helpers.
pub mod audio;

use app::App;
use config::{SoundPreset, SoundVariation};
use daktilo_common::event::DaktiloEvent;
use error::Result;
use rdev::listen;
use std::thread;

/// Starts the typewriter.
pub async fn run(
    sound_presets: Vec<SoundPreset>,
    variation: Option<SoundVariation>,
    device: Option<String>,
) -> Result<()> {
    // Create a listener for the keyboard events.
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<DaktiloEvent>();
    thread::spawn(move || {
        listen(move |event| {
            sender
                .send(event.event_type.into())
                .unwrap_or_else(|e| tracing::error!("could not send event {:?}", e));
        })
        .expect("could not listen events");
    });

    // Create the application state.
    tracing::debug!("{:#?}", sound_presets);
    let mut apps = sound_presets
        .into_iter()
        .map(|sound_preset| App::init(sound_preset, variation.clone(), device.clone()))
        .collect::<Result<Vec<_>>>()?;

    let mut temp_apps = vec![];

    // Handle events.
    loop {
        if let Some(event) = receiver.recv().await {
            match event {
                DaktiloEvent::ActivateTemp(name) => {
                }
                _ => {
                    for app in apps.iter_mut() {
                        app.handle_event(event.clone()).unwrap();
                    }
                }
            }
        }
    }
}

fn handle_event(event: DaktiloEvent, apps: &mut Vec<App>) -> Result<()> {
            for app in apps.iter_mut() {
                app.handle_event(event.clone())?;
            }
    Ok(())
}
