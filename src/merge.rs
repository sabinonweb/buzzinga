use gstreamer::{glib::MainLoop, prelude::*, MessageView};

pub fn merge_content() -> anyhow::Result<()> {
    match gstreamer::init() {
        Ok(()) => log::info!("GStreamer initialized successfully!"),
        Err(e) => log::error!("Error while initializing GStreamer: {:?}", e),
    }

    let version = gstreamer::version();
    log::info!(
        "GStreamer Version: {}.{}.{}.{}",
        version.0,
        version.1,
        version.2,
        version.3
    );

    let source = gstreamer::ElementFactory::make("videotestsrc")
        .name("source")
        .build()
        .unwrap();

    println!(
        "Created source element: {}",
        source.property::<String>("name")
    );

    let sink = gstreamer::ElementFactory::make("videotestsink")
        .name("sink")
        .build()
        .unwrap();

    println!("Created sink: {}", sink.property::<String>("name"));

    let pipeline = gstreamer::Pipeline::default();
    pipeline.add_many(&[&source, &sink]).unwrap();

    source.link(&sink).unwrap();
    let main_loop = MainLoop::new(None, false);

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();
    bus.add_watch(move |_, msg| match msg.view() {
        Message::Eos(..) => {
            log::error!("End of stream reached, quitting...");
            main_loop.quit();
        }
        Message::Error(err) => {
            log::error!("Error: {:?}", err);
            main_loop.quit();
        }
    });

    Ok(())
}
