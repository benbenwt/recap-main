use std::path::PathBuf;

use iced::{
    Element, Length, Subscription, Task, color,
    futures::{SinkExt, Stream, StreamExt},
    stream,
    widget::{self, Row, button, container, row, text, tooltip},
};
use notify::Watcher as _;
use tracing::info;
use uuid::Uuid;

use crate::{
    paths::get_annotation_path,
    sound::double_beep,
    utils::{
        action::{Action, ActionTask},
        play_back_annotations::play_back_annotations,
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Event(notify::Event),
    OpenDir(Uuid),
    OpenVideo(Uuid),
    Remove(Uuid),
    UuidClicked(Uuid),
    RunBack(PathBuf),
    RecordingPerformance(Uuid),
}

#[derive(Debug)]
pub struct State {
    pub files: Vec<(Uuid, PathBuf)>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let files = get_dirs();
        Self { files }
    }
}

pub fn view(state: &crate::App) -> Element<'_, Message> {
    let files = widget::scrollable(
        widget::column(state.recordings.files.iter().map(|file| {
            let mut row: Vec<Element<'_, Message>> = vec![];
            if state.current_uuid == Some(file.0) {
                row.push(widget::text("Recording  ").into());
            }
            let mut has_error = false;
            // Check for runtime errors
            if state.errors.contains_key(&file.0) {
                has_error = true;
                let error = state.errors.get(&file.0).unwrap().join("\n");
                row.push(
                    widget::tooltip(
                        widget::text("Error").color([1.0, 0.0, 0.0]),
                        container(widget::text(error)).padding(10).style(|_| {
                            widget::container::Style::default().background(color!(0x353535))
                        }),
                        widget::tooltip::Position::Bottom,
                    )
                    .into(),
                );
            }
            row.push(file_view(&file.0, has_error).into());
            Row::with_children(row).width(Length::Fill).into()
        }))
        .spacing(4),
    );

    container(widget::column![widget::text("Recordings"), files]).into()
}

fn file_view(uuid: &Uuid, _has_error: bool) -> impl Into<Element<'_, Message>> {
    let mut action_buttons = vec![
        widget::button(
            text(iced_fonts::Bootstrap::GraphUp.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
        )
        .on_press(Message::RecordingPerformance(*uuid))
        .into(),
        widget::button(
            text(iced_fonts::Bootstrap::Play.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
        )
        .on_press(Message::OpenVideo(*uuid))
        .into(),
        widget::button(
            text(iced_fonts::Bootstrap::Folder.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
        )
        .on_press(Message::OpenDir(*uuid))
        .into(),
    ];

    // Add optional playback button
    if cfg!(feature = "playback") {
        action_buttons.push(
            widget::button(
                text(iced_fonts::Bootstrap::FilePlay.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
            )
            .on_press(Message::RunBack(get_annotation_path(uuid)))
            .into(),
        );
    }

    // Add trash button
    action_buttons.push(
        widget::button(
            text(iced_fonts::Bootstrap::Trash.to_string()).font(iced_fonts::BOOTSTRAP_FONT),
        )
        .on_press(Message::Remove(*uuid))
        .into(),
    );

    row![
        tooltip(
            button(widget::text(uuid.to_string())).on_press(Message::UuidClicked(*uuid)),
            container("Click to copy UUID")
                .padding(10)
                .style(|_| widget::container::Style::default().background(color!(0x353535))),
            widget::tooltip::Position::Bottom,
        ),
        Row::with_children(action_buttons).spacing(3)
    ]
    .spacing(6)
    .padding(iced::padding::right(15))
    .wrap()
}

pub fn update(top_state: &mut crate::App, message: Message) -> ActionTask<Message> {
    let state = &mut top_state.recordings;
    match message {
        Message::RecordingPerformance(id) => {
            return crate::Message::SetRecordingPerformance(Some(id)).tat();
        }
        Message::RunBack(file) => {
            return Task::future(async move {
                tracing::debug!("running back: {:?}", file);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                if !file.exists() {
                    tracing::error!("File not found: {:?}", file);
                    double_beep();
                    return;
                }

                if let Err(err) = play_back_annotations(file).await {
                    tracing::error!("Failed to play back annotations: {:?}", err);
                    double_beep();
                }
            })
            .discard()
            .tat();
        }
        Message::OpenVideo(uuid) => {
            if let Some((_, path)) = state.files.iter().find(|(u, _)| u == &uuid) {
                if let Err(err) = open::that(path.join("video.mp4")) {
                    tracing::error!("failed to open video: {:?}", err);
                }
            }
        }
        Message::OpenDir(uuid) => {
            if let Some((_, path)) = state.files.iter().find(|(u, _)| u == &uuid) {
                if let Err(err) = open::that(path) {
                    tracing::error!("failed to open dir: {:?}", err);
                }
            }
        }
        Message::UuidClicked(uuid) => {
            let _ = top_state.clipboard.set_text(uuid.to_string());
        }
        Message::Event(event) => {
            tracing::debug!("event: {:?}", event);
            match event.kind {
                notify::EventKind::Create(_) | notify::EventKind::Remove(_) => {
                    state.files = get_dirs();
                    info!("{:?}", state.files);
                }
                _ => {}
            }
        }
        Message::Remove(file) => {
            tracing::debug!("removing: {:?}", file);
            if file == top_state.current_uuid.unwrap_or_default() {
                tracing::error!("cannot remove current recording");
                return Task::none().tat();
            }
            if let Some((_, path)) = state.files.iter().find(|(uuid, _)| uuid == &file) {
                if let Err(e) = std::fs::remove_dir_all(path) {
                    tracing::error!("failed to remove: {:?}", e);
                }
            }
        }
    }
    Task::none().tat()
}

pub fn subscription(_model: &State) -> Subscription<Message> {
    Subscription::run(watch_recordings)
}

/// Watch for changes in the recordings directory
fn watch_recordings() -> impl Stream<Item = Message> {
    stream::channel(
        10,
        |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
            let (mut tx, mut rx) = iced::futures::channel::mpsc::channel::<Message>(10);

            let mut watcher = notify::recommended_watcher(move |res| match res {
                Ok(event) => {
                    tracing::debug!("event: {:?}", event);
                    let _ = tx.try_send(Message::Event(event));
                }
                Err(e) => tracing::error!("watch error: {:?}", e),
            })
            .unwrap();

            watcher
                .watch(
                    &crate::paths::get_paths().recordings_dir,
                    notify::RecursiveMode::NonRecursive,
                )
                .unwrap();

            while let Some(event) = rx.next().await {
                output.send(event).await.unwrap();
            }
        },
    )
}

fn get_dirs() -> Vec<(Uuid, PathBuf)> {
    let path = &crate::paths::get_paths().recordings_dir;
    let mut dirs = vec![];
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let path = entry.path();
                    if let Ok(uuid) =
                        uuid::Uuid::parse_str(&path.iter().next_back().unwrap().to_string_lossy())
                    {
                        dirs.push((uuid, path));
                    }
                }
            }
        }
    }
    dirs.sort();
    dirs.reverse();
    dirs
}

/// Get the first recording ID (for hotkey playback)
pub fn get_first_recording_id() -> Option<Uuid> {
    get_dirs().first().map(|(uuid, _)| *uuid)
}
