use beryllium::*;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();

    let win_args = video::CreateWinArgs{
        title: "Hello Window",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: true,
        resizable: true
    };

    let win = sdl.create_gl_window(win_args).expect("couldn't make a window and context");

    loop {
        
    } 
}
