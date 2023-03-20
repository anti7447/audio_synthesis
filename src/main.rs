// So that there is no console
#![windows_subsystem = "windows"]

// import frameworks
use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source, Amplify, TakeDuration};
use druid::widget::{Align, Flex, TextBox, Button};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 75.0;
const WINDOW_TITLE: LocalizedString<MainState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct MainState {
    freq: String,
    time: String,
    volume: String,
}

fn main() {

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = MainState {
        freq: "440.0".into(),
        time: "0.1".into(),
        volume: "0.20".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

}

fn build_root_widget() -> impl Widget<MainState> {

    let frequency = TextBox::new()
        .with_placeholder("Frequency")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(MainState::freq);

    let time = TextBox::new()
        .with_placeholder("Time")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(MainState::time);

    let volum = TextBox::new()
        .with_placeholder("Volum")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(MainState::volume);


    let button = Button::new("Play")
        .on_click(|_ctx, data: &mut MainState, _env| {

            let freq_f32: f32 = data.freq.parse().expect("Не удалось пропарсить строку в f32");
            let time_f32: f32 = data.time.parse().expect("Не удалось пропарсить строку в f32");
            let volume_f32: f32 = data.volume.parse().expect("Не удалось пропарсить строку в f32");

            create_sink(freq_f32, time_f32, volume_f32)

        });


    let textboxs_for_sink = Flex::row()
        .with_child(frequency)
        .with_child(time)
        .with_child(volum);

    let layout = Flex::column()
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textboxs_for_sink)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);


    Align::centered(layout)

}

fn create_sink(freq: f32, time: f32, volume: f32) {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(add_source(freq, time, volume));

    sink.sleep_until_end();

}

fn add_source(freq: f32, time: f32, volume: f32) -> Amplify<TakeDuration<SineWave>> {
    SineWave::new(freq).take_duration(Duration::from_secs_f32(time)).amplify(volume)
}