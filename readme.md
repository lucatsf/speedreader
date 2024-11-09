# Speed Reader GNOME

**Speed Reader GNOME** é um aplicativo de leitura rápida projetado para ambientes GNOME. Ele ajuda os usuários a melhorar a velocidade de leitura ao exibir palavras de documentos PDF uma de cada vez no centro da tela, com uma interface amigável e controle de velocidade ajustável.

## Recursos
- **Leitura rápida de PDFs**: Carregue documentos PDF e leia rapidamente uma palavra de cada vez, focando-se em melhorar a velocidade e compreensão.
- **Salvamento automático do progresso**: O aplicativo salva automaticamente sua última posição de leitura para que você possa continuar de onde parou.
- **Controle de Velocidade**: Ajuste o ritmo de leitura (em palavras por minuto - bpm) de acordo com suas preferências.
- **Controle de Leitura**: Avançar, retroceder, pausar ou retomar a leitura facilmente através de botões de controle.
- **Tempo Estimado Restante**: Veja quanto tempo resta para terminar a leitura do documento com base na velocidade atual.
- **Palavra Centralizada**: Cada palavra exibida é centralizada, com a letra do meio destacada em vermelho para ajudar o usuário a manter o foco.

## Tecnologias Utilizadas
- **Rust**: Linguagem principal usada para desenvolver o aplicativo.
- **GTK 4**: Biblioteca gráfica utilizada para construir a interface do usuário.
- **Poppler**: Biblioteca usada para extrair texto de arquivos PDF.
- **Serde**: Usada para serializar e desserializar o estado da leitura.

## Instalação e Execução
Siga os passos abaixo para instalar e executar o aplicativo no seu ambiente Linux.

**Nota**: Este aplicativo foi desenvolvido e testado em um ambiente GNOME. Pode não funcionar corretamente em outros ambientes de desktop.

faça o download do arquivo zip e extraia o conteúdo aqui [speedreader.zip](https://github.com/lucatsf/speedreader/releases/download/v1.0.0/speedreader.zip)
```sh
unzip speedreader.zip && \
cd build/ && \
chmod +x install.sh && \
sudo ./install.sh
```

Desinstalar o aplicativo
```sh
./uninstall.sh
```

### Dependências
Este projeto possui algumas dependências externas que devem ser instaladas antes de compilá-lo. Certifique-se de ter:
- **Rust**: O compilador e gerenciador de pacotes do Rust. Instalação recomendada via [rustup](https://rustup.rs/).
- **GTK 4** e **Poppler**: Certifique-se de ter as bibliotecas GTK 4 e Poppler instaladas em seu sistema.
  - No Debian/Ubuntu, você pode instalá-las com o comando:
    ```sh
    sudo apt-get install libgtk-4-dev libpoppler-glib-dev
    ```

### Clonando o Repositório
Clone o repositório do projeto:
```sh
git clone github.com:lucatsf/speedreader.git
cd speedreader
```

### Compilando e Executando
Compile e execute o projeto utilizando o cargo:
```sh
cargo run
```

## Como Usar
1. **Abrir o Aplicativo**: Execute o aplicativo usando `cargo run` ou abra o binário gerado.
2. **Carregar um PDF**: Clique em **"Selecionar PDF"** e escolha o documento que deseja ler.
3. **Ajustar a Velocidade**: Utilize o seletor de velocidade para definir o ritmo de leitura (em bpm).
4. **Controles de Navegação**:
   - **Play/Pause**: Clique no botão de "play/pause" para iniciar ou pausar a leitura.
   - **Avançar/Retroceder**: Use os botões de avançar (>>) ou retroceder (<<) para navegar entre as palavras manualmente.
5. **Visualizar Tempo Restante**: O tempo estimado restante de leitura será mostrado na parte inferior.

## Contribuições
Contribuições são bem-vindas! Se você quiser melhorar este projeto, siga os passos:
1. Faça um fork do repositório.
2. Crie uma nova branch com suas modificações.
3. Envie um pull request descrevendo suas alterações.

Certifique-se de seguir o estilo de código estabelecido e documentar suas alterações de forma adequada.

## Licença
Este projeto está licenciado sob a **MIT License**. Sinta-se à vontade para usar, modificar e distribuir à vontade.

## Capturas de Tela
Adicione aqui algumas capturas de tela do aplicativo em funcionamento para mostrar a interface.

## Contato
Se você tiver dúvidas ou sugestões, entre em contato:
- **Nome**: lucatsf
- **Email**: [lucastorresfellicio@gmail.com](lucastorresfellicio@gmail.com)

Fique à vontade para enviar feedbacks ou sugestões de melhorias!

---
Espero que esse README dê uma boa introdução ao projeto. Se precisar de alguma outra seção ou tiver outras ideias, é só me avisar!