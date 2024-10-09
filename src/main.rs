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
    // Inicializa o render
    let mut render = Render::new(&window).await?;
    
    // Carregar a textura de uma imagem na pasta "assets/images"
    let (_, sprite_bind_group) = render.load_texture("src/assets/images/image.jpg")?;
    
    // Iniciar o loop de eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    render.resize(physical_size);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Chame aqui a função de renderização de sprite
                if let Err(e) = render.render() {
                    eprintln!("Render error: {:?}", e);
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
