# Razor-base (AINDA EM DESENVOLVIMENTO)

Razor-base é a fundação modular e escalável para o desenvolvimento de jogos em Rust, projetada para ser o núcleo de frameworks e engines mais complexas. Este projeto oferece os sistemas essenciais para renderização 2D e 3D, gerenciamento de física, ECS (Entity Component System), input, e áudio.

O Razor-base serve como ponto de partida para a construção de frameworks e engines customizadas, oferecendo alta performance e flexibilidade. Ele é leve e modular, permitindo que desenvolvedores expandam ou ajustem o código para suas necessidades específicas.
Principais Funcionalidades:

    Renderização 2D/3D: Sistema gráfico utilizando wgpu, permitindo renderizações eficientes e multi-plataforma.
    
    Física 2D/3D: Suporte a simulações físicas, utilizando bibliotecas como rapier para oferecer colisão e movimentação realistas.
    
    ECS (Entity Component System): Sistema para gerenciar entidades e seus componentes, facilitando o desenvolvimento de grandes sistemas.
    
    Input: Gerenciamento de eventos de teclado, mouse e gamepads, utilizando a biblioteca winit.
    
    Áudio: Sistema básico para gerenciamento de som e música, utilizando rodio.
    
    Modularidade: Projete seus próprios sistemas ou adicione novos módulos de acordo com a necessidade.

Objetivo do Projeto:

O Razor-base é a fundação técnica para a criação de frameworks e engines de jogos. Ele será utilizado como a base para dois futuros projetos:

    Razorlib: Um framework de desenvolvimento de jogos em Rust, mais completo e otimizado, similar ao libGDX.
    
    Razor Engine: Uma engine de jogos completa com editor visual, facilitando o desenvolvimento de jogos para diversas plataformas.

Como Usar:
Requisitos:

    Rust (versão 1.60 ou superior)
    Cargo

Instalação:

Clone este repositório e instale as dependências com o Cargo.

# bash
```
git clone https://github.com/seu-usuario/razor-base.git
cd razor-base
cargo build
```

Exemplos:

Aqui estão exemplos de como utilizar os principais sistemas (renderização, física, ECS, etc.). [Em construção]

Contribuições:

Contribuições são bem-vindas! Se você quiser ajudar no desenvolvimento ou sugerir melhorias, fique à vontade para abrir uma issue ou enviar um pull request.
