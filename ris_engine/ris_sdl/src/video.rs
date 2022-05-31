use sdl2::Sdl;

pub struct Video {
    window: sdl2::video::Window,
}

impl Video {
    pub fn new(sdl_context: Sdl) -> Result<Video, Box<dyn std::error::Error>> {
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("ris_engine", 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let video = Video { window };

        Ok(video)
    }
}
