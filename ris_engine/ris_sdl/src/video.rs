use sdl2::{video::Window, Sdl};

pub struct Video {
    _window: Window,
}

impl Video {
    pub fn new(sdl_context: &Sdl) -> Result<Video, String> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("ris_engine", 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let video = Video { _window: window };

        Ok(video)
    }
}
