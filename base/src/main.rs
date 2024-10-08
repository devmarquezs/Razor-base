use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use anyhow::Result; // Para lidar com erros
mod graphics;
use graphics::render::Render;

fn main() -> Result<()> {
    // Criar o event loop e a janela
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    // Rodar a função assíncrona principal
    pollster::block_on(run(event_loop, window))
}

async fn run(event_loop: EventLoop<()>, window: winit::window::Window) -> Result<()> {
    // Inicializar o renderer
    let mut render = Render::new(&window).await?;

    // Iniciar o loop principal de eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    render.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    render.resize(*new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if let Err(e) = render.render() {
                    eprintln!("Render error: {:?}", e);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw(); // Solicita uma nova renderização
            }
            _ => {}
        }
    });
}
