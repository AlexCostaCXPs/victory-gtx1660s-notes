# Visão geral

## Por que um jogo de Xbox não roda direto no PC

Um console tem um processador e uma placa de vídeo próprios, com um sistema operacional e bibliotecas específicas. Um PC tem outros componentes. Para rodar o jogo, é preciso reproduzir o que o console oferece, sem mudar o código do jogo.

## O que o projeto faz

O VIctory é uma camada de compatibilidade: ele executa o código do título e traduz o que o console faria para o Windows. Nada do jogo é distribuído aqui; o usuário precisa da sua própria cópia legítima.

## O que medimos nesta investigação

- Em 1280x720, no perfil Series S, o jogo chegou ao mundo, com imagem boa em um quadro observado.
- Ficou estável por 10 minutos, a cerca de 2 FPS, com 4.400 a 6.200 desenhos por quadro.
- Não é jogável ainda: entradas além do menu não foram testadas.

Veja o README para a tabela de status, as falhas observadas e as limitações.