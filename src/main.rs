use winit::{
    event_loop::{ControlFlow,EventLoop},
    window::WindowBuilder,
};

use anyhow::Result; // Para lidar com erros
use graphics::render::Render;
mod graphics;


fn main() -> Result<()> {
    // Criar o event loop e a janela
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    // Rodar a função assíncrona principal
    pollster::block_on(run(event_loop, window))
}

async fn run(event_loop: EventLoop<()>, window: winit::window::Window) -> Result<()> {
    // Inicializar o renderizador
    let mut render = Render::new(&window).await?;
    
    // Carregar a textura
    let (_, bind_group) = render.load_texture("src/assets/images/Razor-base.png")?;
    
    // Iniciar o loop de eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;  // Define para verificar os eventos constantemente
    
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;  // Fechar a janela corretamente
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    // Redimensionar a janela
                    render.resize(physical_size);
                }
                _ => {}
            },
            winit::event::Event::RedrawRequested(_) => {
                if let Err(e) = render.render(&bind_group) {
                    eprintln!("Render error: {:?}", e);
                }
            }
            _ => {}
        }
    });
    
}    
