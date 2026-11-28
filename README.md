# Victory on a GeForce GTX 1660 SUPER: performance and stability notes

[Português](#português) · [English](#english)

---

## Português

Notas de uma investigação de desempenho e estabilidade do Victory (runtime que roda o código de um jogo de console x86-64 num PC com Windows) em uma GeForce GTX 1660 SUPER (6 GB de VRAM), com 16 GB de RAM, no Windows 11.

**Este repositório não contém o jogo, arquivos de jogo, cache de shaders, executáveis ou logs com caminhos pessoais.** É preciso ter a sua própria cópia legítima do jogo.

### Resumo

- **Resultado final:** o jogo não passou da fase de abertura de forma estável nesta configuração. A cena do mundo carrega, mas a imagem completa não aparece e o jogo cai.
- **Causa principal observada:** falta de memória da placa. O título pede mais memória de textura do que os 6 GB de VRAM aguentam, e uma operação do driver falha (`0x8007000E`).
- **Decisão:** parar por aqui nesta configuração. Os motivos estão abaixo.

### Hardware e base de software

- **Máquina:** Windows 11, GeForce GTX 1660 SUPER (6 GB), 16 GB de RAM, Intel Core i5-10400F.
- **Base:** o projeto parte do [Marvisak/VIctory](https://github.com/Marvisak/VIctory), sob GPL v3 (ver `LICENSE`). Mudanças e diagnósticos próprios também seguem GPL v3.
- **Projetos consultados (não portados):** [uiliannn/VItoriaBr](https://github.com/uiliannn/VItoriaBr), [fxceless/victory-fork](https://github.com/fxceless/victory-fork), [trustno1-coder/VIctory-NVIDIA](https://github.com/trustno1-coder/VIctory-NVIDIA) (fork que exige 32 GB de RAM), [stellarr-studio/kelvaroq](https://github.com/stellarr-studio/kelvaroq), [Dufftwt/VIctory](https://github.com/Dufftwt/VIctory), [Quat3rnion/VIctory](https://github.com/Quat3rnion/VIctory) e [Marvisak/VIctory PR #32](https://github.com/Marvisak/VIctory/pull/32).

### Evoluções medidas

| Medição | Resultado |
|---|---|
| Passe de texturas com `VICTORY_TEXTURE_PASS_MS=50` (antes 250) | passe caiu de 225–273 ms para 55–57 ms; quadro médio no primeiro minuto da intro caiu de 1.892 para 1.451 ms. **Único ganho medido e confirmado.** |
| Cópia de volta de buffers (diagnóstico) | cerca de 81% do tempo da cópia está na comparação dos buffers na CPU; mais de 99% dos bytes comparados já eram iguais |
| Candidatos a excluir da comparação | 99,7% dos bytes candidatos estavam cobertos por intervalos que precisam ser comparados; ganho estimado de cerca de 0,005% dos bytes |
| Função `differing` alternativa (blocos de 1 MiB) | igual com poucas alterações, mais lenta com muitas |
| Limite de cópias na placa de 1024 MB (`VICTORY_CARD_BUFFERS_MB`) | sem ganho confirmado; a placa ficou perto do limite |

### Testes que não funcionaram ou não ajudaram

| Teste | Resultado |
|---|---|
| `VICTORY_WIDE_REFRESH=1` | preparo de cópias caiu, mas o quadro subiu de 1.698 para 2.318 ms |
| `VICTORY_WIDE_MERGE=1` | caiu em cerca de 30 s com `0xC0000409` |
| `VICTORY_FILL_WORKERS=3` | caiu em cerca de 2,5 min com `0xC0000409`; teste com 1 worker não mostrou ganho claro |
| `VICTORY_CARD_BUFFERS_MIN_MB=1` | sem ganho claro; faixa de 1,2–1,5 FPS |
| `VICTORY_KEEP_SNAPSHOTS=1` com 256 MB | sem ganho visível; a RAM do sistema chegou a 7 MB livres |
| `VICTORY_DISPLAY_MODE=1280x720` | não muda a resolução de renderização; o título apresentou em 3840x2160 |
| Perfil de console Series S (`--console series-s`) | apresentou em 1920x1080; a tela continuou sem imagem completa |
| Patch 0045 do Kelvaroq (espera de observação) | não se aplica à nossa árvore sem portar o 0044 antes; deixado de fora |
| Correção da issue #721 (escrever só bytes alterados) | mais lenta na bancada quando há muitas alterações |
| Limite de views (contador de índices acima de 1 milhão) | o contador só fica ativo com `VICTORY_RAW_VIEWS=0`; a sessão com ele caiu por falta de memória antes de gerar dados válidos |
| Ray tracing do título (`VICTORY_BVH_BUILDER=1`) | não testado por causa do risco de travar a placa |

### Falhas observadas

- **`0xC0000005` em `victory-image-…dll+0x23CF683`:** acesso a endereço nulo dentro do código do título. Apareceu em várias execuções, com configurações diferentes. Sem símbolos, não há nome de função.
- **`0x80000003` em `victory-image-…dll+0x5149aec`:** parada numa cadeia de chamadas do título. A mensagem de assert não aparece no log. Nosso código tem um ponto de parada, mas só dispara em imports inexistentes, e isso não ocorreu.
- **`0x887A0005` (dispositivo de GPU removido):** ocorreu depois de erros de gravação do DirectStorage e de novos pipelines de shader. Em uma execução, veio cerca de 85 segundos depois dos erros de gravação.
- **`0xC0000005` no driver da NVIDIA (`nvwgf2umx.dll+0x422e03`):** apareceu em duas execuções, com e sem o contador de views. A pilha passa pelo caminho de submissão do driver.
- **`0x8007000E` (memória insuficiente):** a análise da última queda mostra a memória da placa acima do orçamento, com o título pedindo cerca de 11 GB de texturas numa placa de 6 GB.
- A camada de depuração do Direct3D 12 foi instalada (Ferramentas de Gráficos do Windows), mas nenhuma execução mostrou o comando que falhou.

### Limitações

- O jogo fica em cerca de 1 FPS na GTX 1660 SUPER, com a cena do mundo montando, mas sem imagem completa.
- Não há símbolos da imagem do título, então a função que falha não foi identificada.
- A captura de minidump local cobre exceções do código do jogo, não falhas no host.
- Os números vêm de sessões curtas; não há medições em outra máquina.
- A tradução do menu F1 para português existe em uma cópia de trabalho e funciona (o menu abre em português). O código dela não foi publicado aqui.

### Por que paramos

- A memória do sistema (16 GB) fica abaixo do que o fork pede (32 GB).
- A VRAM de 6 GB não aguenta as texturas que o título pede, mesmo com os limites de cópia.
- Os testes de imagem e de desempenho ficaram incompletos, e as quedas continuaram nas duas causas observadas (acesso ao título e falta de memória da placa).

### O que faria diferente com outra máquina

1. Subir a RAM para 32 GB e usar uma placa com mais VRAM (8 GB ou mais).
2. Repetir a investigação com símbolos da imagem do título, ou com um depurador configurado desde o início.
3. Só então medir desempenho, com uma cena fixa e captura de quadros.

### Crédito

Os diagnósticos e medições desta investigação são próprios. As ideias de opções de ambiente e a base do código vêm do Victory e dos projetos citados acima.

---

## English

Notes from a performance and stability investigation of Victory (a runtime that runs an x86-64 console game on Windows) on a GeForce GTX 1660 SUPER (6 GB VRAM), 16 GB RAM, Windows 11.

**This repository contains no game, no game files, no shader cache, no executables and no logs with personal paths.** You need your own legitimate copy of the game.

### Summary

- **Final result:** the game did not get past the opening stage stably on this setup. The world scene loads, but the full picture never appears and the game crashes.
- **Main cause observed:** GPU memory shortage. The title asks for more texture memory than 6 GB of VRAM can hold, and a driver operation fails (`0x8007000E`).
- **Decision:** stop here on this setup. The reasons are below.

### Hardware and software base

- **Machine:** Windows 11, GeForce GTX 1660 SUPER (6 GB), 16 GB RAM, Intel Core i5-10400F.
- **Base:** the project starts from [Marvisak/VIctory](https://github.com/Marvisak/VIctory), under GPL v3 (see `LICENSE`). Our own changes and diagnostics are also GPL v3.
- **Projects reviewed (not ported):** [uiliannn/VItoriaBr](https://github.com/uiliannn/VItoriaBr), [fxceless/victory-fork](https://github.com/fxceless/victory-fork), [trustno1-coder/VIctory-NVIDIA](https://github.com/trustno1-coder/VIctory-NVIDIA) (a fork that asks for 32 GB RAM), [stellarr-studio/kelvaroq](https://github.com/stellarr-studio/kelvaroq), [Dufftwt/VIctory](https://github.com/Dufftwt/VIctory), [Quat3rnion/VIctory](https://github.com/Quat3rnion/VIctory) and [Marvisak/VIctory PR #32](https://github.com/Marvisak/VIctory/pull/32).

### Measured improvements

| Measurement | Result |
|---|---|
| Texture pass with `VICTORY_TEXTURE_PASS_MS=50` (was 250) | pass dropped from 225–273 ms to 55–57 ms; average frame in the first minute of the intro dropped from 1,892 to 1,451 ms. **The only confirmed gain.** |
| Copy-back of buffers (diagnostic) | about 81% of copy-back time is the CPU comparison of buffers; over 99% of compared bytes were already equal |
| Candidates to skip in the comparison | 99.7% of candidate bytes were covered by ranges that still must be compared; estimated gain about 0.005% of bytes |
| Alternative `differing` function (1 MiB blocks) | equal with few changes, slower with many changes |
| Card copy limit at 1024 MB (`VICTORY_CARD_BUFFERS_MB`) | no confirmed gain; the card stayed near its limit |

### Tests that failed or did not help

| Test | Result |
|---|---|
| `VICTORY_WIDE_REFRESH=1` | copy preparation dropped, but the frame rose from 1,698 to 2,318 ms |
| `VICTORY_WIDE_MERGE=1` | crashed after about 30 s with `0xC0000409` |
| `VICTORY_FILL_WORKERS=3` | crashed after about 2.5 min with `0xC0000409`; the run with 1 worker showed no clear gain |
| `VICTORY_CARD_BUFFERS_MIN_MB=1` | no clear gain; 1.2–1.5 FPS range |
| `VICTORY_KEEP_SNAPSHOTS=1` with 256 MB | no visible gain; system RAM dropped to 7 MB free |
| `VICTORY_DISPLAY_MODE=1280x720` | does not change the render resolution; the title presented at 3840x2160 |
| Series S console profile (`--console series-s`) | presented at 1920x1080; the picture still did not fully appear |
| Kelvaroq patch 0045 (observation wait) | does not apply to our tree without porting 0044 first; left out |
| Issue #721 fix (write back only changed bytes) | slower in the benchmark when there are many changes |
| View-index limit counter (indices above 1 million) | the counter only works with `VICTORY_RAW_VIEWS=0`; the run with it crashed from memory shortage before producing valid data |
| Title ray tracing (`VICTORY_BVH_BUILDER=1`) | not tested because of the risk of hanging the card |

### Observed failures

- **`0xC0000005` at `victory-image-…dll+0x23CF683`:** null-address write inside the title's code. It appeared in several runs with different settings. Without symbols there is no function name.
- **`0x80000003` at `victory-image-…dll+0x5149aec`:** a stop inside a chain of title calls. The assert message does not appear in the log. Our code has a stop point, but it only fires on missing imports, which did not happen.
- **`0x887A0005` (GPU device removed):** happened after DirectStorage write errors and new shader pipelines. In one run it came about 85 seconds after the write errors.
- **`0xC0000005` in the NVIDIA driver (`nvwgf2umx.dll+0x422e03`):** appeared in two runs, with and without the view counter. The stack goes through the driver's submission path.
- **`0x8007000E` (out of memory):** analysis of the last crash shows card memory above its budget, with the title asking for about 11 GB of textures on a 6 GB card.
- The Direct3D 12 debug layer was installed (Windows Graphics Tools), but no run showed the failing command.

### Limitations

- The game runs at about 1 FPS on the GTX 1660 SUPER, with the world scene loading but without the full picture.
- There are no symbols for the title image, so the failing function was not identified.
- The local minidump capture covers exceptions in the game's code, not host failures.
- The numbers come from short sessions; there are no measurements on another machine.
- A Portuguese translation of the F1 menu exists in a working copy and works (the menu opens in Portuguese). Its code is not published here.

### Why we stopped

- System RAM (16 GB) is below what the fork asks for (32 GB).
- 6 GB of VRAM does not hold the textures the title asks for, even with copy limits.
- Image and performance tests stayed incomplete, and the crashes continued from both observed causes (title code access and card memory shortage).

### What I would do differently on another machine

1. Raise RAM to 32 GB and use a card with more VRAM (8 GB or more).
2. Repeat the investigation with symbols for the title image, or with a debugger set up from the start.
3. Only then measure performance, with a fixed scene and frame capture.

### Credits

The diagnostics and measurements in this investigation are our own. The environment option ideas and the code base come from Victory and the projects listed above.
