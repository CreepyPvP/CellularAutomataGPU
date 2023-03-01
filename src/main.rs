use std::fs;

extern crate gl;
use gl::types::*;
use glfw::{Context, WindowEvent, Key, Action, CursorMode, WindowHint};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::ContextVersion(4, 5));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(
        800,
        600,
        "Cool opengl window",
        glfw::WindowMode::Windowed
    ).unwrap();

    // window.set_cursor_mode(CursorMode::Hidden);
    window.set_cursor_mode(CursorMode::Disabled);
    window.make_current();

    // window.set_all_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    window.show();

    // Opengl stuff...
    gl::load_with(|s| glfw.get_proc_address_raw(s));
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        let compute_shader = gl::CreateShader(gl::COMPUTE_SHADER);
        let shader_source = fs::read_to_string("shaders/compute.c").unwrap();

        gl::ShaderSource(
            compute_shader, 
            1, 
            &(shader_source.as_bytes().as_ptr().cast()),
            &(shader_source.len().try_into().unwrap())
        );
        gl::CompileShader(compute_shader);
        
        let mut success = 0;
        gl::GetShaderiv(compute_shader, gl::COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                compute_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, compute_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        gl::DeleteShader(compute_shader);

        let mut ssbo = 0;
        gl::GenBuffers(1, &mut ssbo);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo);
        const BUFFER_DATA: [f32; 1] = [100.0];
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER, 
            std::mem::size_of_val(&BUFFER_DATA) as isize,
            BUFFER_DATA.as_ptr().cast(),
            gl::DYNAMIC_COPY
        );
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);

        let mut ssbo1 = 0;
        gl::GenBuffers(1, &mut ssbo1);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, ssbo1);
        const BUFFER_DATA1: [f32; 1] = [0.0];
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER, 
            std::mem::size_of_val(&BUFFER_DATA1) as isize,
            BUFFER_DATA1.as_ptr().cast(),
            gl::DYNAMIC_COPY
        );
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);

        gl::UseProgram(shader_program);

        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, ssbo);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, ssbo1);

        gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
        gl::DispatchCompute(1, 1, 1);

        let mut output_data: [f32; 1] = [0.0];
        gl::GetBufferSubData(gl::SHADER_STORAGE_BUFFER, 0, 4, output_data.as_mut_ptr().cast());
        println!("{}", output_data[0]);
    };

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }
    }
}
