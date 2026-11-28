# VIctory na GeForce GTX 1660 SUPER / VIctory on a GeForce GTX 1660 SUPER

[🇧🇷 Português](#português) · [🇺🇸 English](#english)

> **Base e licença:** este trabalho parte do projeto open-source [Marvisak/VIctory](https://github.com/Marvisak/VIctory), sob **GPL-3.0-or-later** ([LICENSE](LICENSE)). Mudanças e diagnósticos próprios seguem a mesma licença. **Este repositório não contém nem distribui nenhum arquivo de jogo**, cache de shaders, executável ou log com caminhos pessoais. Veja [docs/politica-de-arquivos.md](docs/politica-de-arquivos.md).

---

## Português

### Status e progresso

| Etapa | Status | Evidência e observações |
|---|---|---|
| Preparação (instalação, configuração) | Concluído | Instruções em "Primeiros passos" abaixo |
| Executável rodando | Concluído | Inicia com o perfil Series S em 1280x720 |
| Primeiro frame | Concluído | Os primeiros quadros saem pretos |
| Menu principal | Concluído | Tela de seleção de conta observada em capturas |
| No mundo | Parcial | Cena de mundo alcançada, com mais de 900 desenhos por quadro; quadros de mundo iniciais pretos |
| Jogável | Não verificado | Não testamos entradas além do menu |
| História | Não verificado | Não testado |
| Gráficos | Parcial | Um quadro de mundo (corredor de presídio) com boa imagem; quadros não comparados entre si |
| Desempenho | Medido | Cerca de 2 FPS (1,4 a 2,5) em 10 minutos, em 1280x720, com 4.400 a 6.200 desenhos por quadro |
| Estabilidade | Parcial | Sem queda nem erro de GPU nos primeiros 10 minutos; depois o log parou de ser escrito, então erros posteriores não foram verificados |

### Resultado atual

No perfil Series S em 1280x720, o jogo chegou ao mundo, com imagem boa em um quadro observado, e ficou estável por mais de 10 minutos a cerca de 2 FPS. **Ainda não é jogável**: não testamos entradas além do menu, e os primeiros quadros de mundo saíram pretos.

### O que medimos

- **Passe de texturas com `VICTORY_TEXTURE_PASS_MS=50`:** passe caiu de 225–273 ms para 55–57 ms; quadro médio no primeiro minuto da intro caiu de 1.892 para 1.451 ms. Único ganho confirmado.
- **Cópia de volta de buffers:** cerca de 81% do tempo está na comparação dos buffers na CPU; mais de 99% dos bytes comparados já eram iguais.
- **Candidatos a excluir da comparação:** 99,7% dos bytes candidatos estavam cobertos por intervalos que precisam ser comparados.
- **Função `differing` alternativa (blocos de 1 MiB):** igual com poucas alterações, mais lenta com muitas.

### O que não funcionou ou não ajudou

- `VICTORY_WIDE_REFRESH=1`: o preparo de cópias caiu, mas o quadro subiu de 1.698 para 2.318 ms.
- `VICTORY_WIDE_MERGE=1`: caiu em cerca de 30 s com `0xC0000409`.
- `VICTORY_FILL_WORKERS=3`: caiu em cerca de 2,5 min com `0xC0000409`.
- `VICTORY_CARD_BUFFERS_MIN_MB=1` e `VICTORY_CARD_BUFFERS_MB=1024`: sem ganho confirmado.
- `VICTORY_KEEP_SNAPSHOTS=1` com 256 MB: sem ganho visível; a RAM do sistema chegou a 7 MB livres.
- `VICTORY_DISPLAY_MODE=1280x720`: não muda a resolução de renderização; o título apresentou em 3840x2160.
- Perfil `--console series-s`: resolução de apresentação de 1920x1080 nas primeiras sessões; a tela ainda não estava completa.
- Patch 0045 do Kelvaroq e correção da issue #721: não se aplicaram limpos à nossa árvore, ou ficaram mais lentos nas medições.
- `VICTORY_RAW_VIEWS=0` com o contador de views: a sessão caiu por falta de memória antes de gerar dados válidos.

### Falhas observadas

- **`0xC0000005` em `victory-image-…dll+0x23CF683`:** escrita em endereço nulo dentro do código do título. Apareceu em várias execuções. Sem símbolos, não há nome de função.
- **`0x80000003` em `victory-image-…dll+0x5149aec`:** parada numa cadeia de chamadas do título. A mensagem do assert não aparece no log.
- **`0x887A0005` (dispositivo de GPU removido):** ocorreu depois de erros de gravação do DirectStorage e de novos pipelines de shader.
- **`0xC0000005` no driver NVIDIA (`nvwgf2umx.dll+0x422e03`):** apareceu em duas execuções, com e sem o contador de views.
- **`0x8007000E` (memória insuficiente):** a memória da placa passou do orçamento; nas sessões mais recentes não apareceu com o limite de texturas em 2048 MB.

### Limitações

- Em 1280x720, com Series S, o jogo fica em cerca de 2 FPS, com 4.400 a 6.200 desenhos por quadro.
- Não há símbolos da imagem do título, então a função que falha não foi identificada.
- A captura de minidump local cobre exceções do código do jogo, não falhas do host.
- A medição de estabilidade cobre 10 minutos; o log parou de ser escrito depois disso.
- Os números vêm de sessões curtas, sem medições em outra máquina.

### Por que paramos antes, e o que mudou

Antes, a decisão era parar, porque a memória do sistema (16 GB) fica abaixo do que o fork pede (32 GB) e a VRAM de 6 GB não comportava as texturas. Depois, a configuração do Series S em 720p chegou ao mundo com imagem boa num quadro e ficou estável por 10 minutos. Por isso a decisão foi revista: o trabalho segue em medição.

### Primeiros passos

1. Tenha a sua própria cópia legítima do jogo. Este repositório não a fornece.
2. Use Windows 10 ou 11 com uma placa NVIDIA e um driver recente. Tenha pelo menos 16 GB de RAM.
3. Compile o runtime a partir de um checkout limpo do projeto base e rode a partir da pasta com a sua cópia do jogo.
4. O primeiro início de uma build nova reinicia sozinho uma vez (aquecimento). Isso é esperado.

### Comandos e variáveis usados

| Comando | O que faz |
|---|---|
| `run <game_scarlett_master.exe> --execute` | inicia o título a partir da pasta do seu jogo |
| `run ... --execute --console series-s` | pede ao título o perfil Series S (1280x720 nas medições desta página) |
| `cargo test -p victory-shader` | roda os testes do tradutor de shaders |
| `cargo test -p victory-gpu` | roda os testes do host de GPU |

Variáveis usadas no melhor resultado: `VICTORY_DISPLAY_MODE=1280x720`, `VICTORY_CAPTURE` e `VICTORY_CAPTURE_WORLD=1` (diagnóstico). Padrão mantido: `VICTORY_COUNT_VIEWS` desligado, `VICTORY_RAW_VIEWS` não definido, `VICTORY_SNAPSHOT_POOL` 1024, `VICTORY_TEXTURE_BUDGET_MB` 2048, MFG desligado e sem downloads. Variáveis que o código do fork não lê: `VICTORY_TEXTURE_PASS_MS` e `VICTORY_FILL_WORKERS`; não tiveram efeito.

### Depuração

- **Crash log:** `%LOCALAPPDATA%\victory\data\<title-id>\crashes.log`, com código da exceção, endereço e pilha.
- **Minidumps:** gravados para exceções no código do jogo (Windows Error Reporting). Falhas do host podem não gerar dump.
- **Camada de depuração do Direct3D 12:** instale "Ferramentas de Gráficos" (`Add-WindowsCapability -Online -Name 'Tools.Graphics.DirectX~~~~0.0.1.0'`, como administrador, e reinicie). Com `VICTORY_D3D12_DEBUG=1`, um build de diagnóstico pode nomear o comando que falha; nas nossas execuções, isso não apareceu.
- **Depurador:** o WinDbg Preview traz o `cdb.exe`. Anexar um depurador mudou o tempo de execução nos nossos testes, então os resultados não são diretamente comparáveis.

### Documentação

- [Arquitetura](docs/arquitetura.md)
- [Visão geral](docs/visao-geral.md)
- [Política de arquivos do jogo](docs/politica-de-arquivos.md)

### Como contribuir

Envie uma configuração testada pelo modelo de issue "Relato de teste" e veja [CONTRIBUTING.md](CONTRIBUTING.md). Não envie arquivos do jogo, cache, executáveis nem chaves.

### Crédito

Os diagnósticos e medições desta investigação são próprios. As ideias de variáveis de ambiente e a base do código vêm do Victory e dos projetos citados em [docs/visao-geral.md](docs/visao-geral.md).

---

## English

### Status and progress

| Stage | Status | Evidence and notes |
|---|---|---|
| Preparation (setup, configuration) | Done | See "Getting started" below |
| Executable running | Done | Starts with the Series S profile at 1280x720 |
| First frame | Done | The first frames come out black |
| Main menu | Done | Account selection screen observed in captures |
| In the world | Partial | World scene reached, with more than 900 draws per frame; the first world frames are black |
| Playable | Not verified | No input beyond the menu was tested |
| Story | Not verified | Not tested |
| Graphics | Partial | One world frame (a prison corridor) looked correct; frames were not compared with each other |
| Performance | Measured | About 2 FPS (1.4 to 2.5) over 10 minutes at 1280x720, with 4,400 to 6,200 draws per frame |
| Stability | Partial | No crash and no GPU error in the first 10 minutes; the log stopped being written after that, so later errors were not checked |

### Current result

On the Series S profile at 1280x720, the game reached the world, with a good picture in one observed frame, and stayed stable for more than 10 minutes at about 2 FPS. **It is not playable yet**: no input beyond the menu was tested, and the first world frames came out black.

### What we measured

- **Texture pass with `VICTORY_TEXTURE_PASS_MS=50`:** the pass dropped from 225–273 ms to 55–57 ms; the average frame in the first minute of the intro dropped from 1,892 to 1,451 ms. The only confirmed gain.
- **Buffer copy-back:** about 81% of the time is the CPU comparison of buffers; over 99% of the compared bytes were already equal.
- **Candidates to skip in the comparison:** 99.7% of candidate bytes were covered by ranges that still must be compared.
- **Alternative `differing` function (1 MiB blocks):** equal with few changes, slower with many.

### What did not work or did not help

- `VICTORY_WIDE_REFRESH=1`: copy preparation dropped, but the frame rose from 1,698 to 2,318 ms.
- `VICTORY_WIDE_MERGE=1`: crashed after about 30 s with `0xC0000409`.
- `VICTORY_FILL_WORKERS=3`: crashed after about 2.5 min with `0xC0000409`.
- `VICTORY_CARD_BUFFERS_MIN_MB=1` and `VICTORY_CARD_BUFFERS_MB=1024`: no confirmed gain.
- `VICTORY_KEEP_SNAPSHOTS=1` with 256 MB: no visible gain; system RAM dropped to 7 MB free.
- `VICTORY_DISPLAY_MODE=1280x720`: does not change the render resolution; the title presented at 3840x2160.
- Series S profile (`--console series-s`): 1920x1080 presentation in the first sessions; the picture was not complete yet.
- Kelvaroq patch 0045 and the issue #721 fix: did not apply cleanly to our tree, or were slower in our measurements.
- `VICTORY_RAW_VIEWS=0` with the view counter: the session crashed from memory shortage before producing valid data.

### Observed failures

- **`0xC0000005` at `victory-image-…dll+0x23CF683`:** null-address write inside the title's code. It appeared in several runs. Without symbols there is no function name.
- **`0x80000003` at `victory-image-…dll+0x5149aec`:** a stop inside a chain of title calls. The assert message does not appear in the log.
- **`0x887A0005` (GPU device removed):** happened after DirectStorage write errors and new shader pipelines.
- **`0xC0000005` in the NVIDIA driver (`nvwgf2umx.dll+0x422e03`):** appeared in two runs, with and without the view counter.
- **`0x8007000E` (out of memory):** the card's memory went over its budget; in the latest sessions it did not appear with the texture limit at 2048 MB.

### Limitations

- On Series S at 1280x720, the game runs at about 2 FPS, with 4,400 to 6,200 draws per frame.
- There are no symbols for the title image, so the failing function was not identified.
- The local minidump capture covers exceptions in the game's code, not host failures.
- The stability measurement covers 10 minutes; the log stopped being written after that.
- The numbers come from short sessions, with no measurements on another machine.

### Why we stopped earlier, and what changed

The earlier decision was to stop, because system RAM (16 GB) is below what the fork asks for (32 GB) and 6 GB of VRAM did not hold the textures. Later, the Series S profile at 720p reached the world with a good picture in one frame and stayed stable for 10 minutes. So the decision was revised, and measurement continues.

### Getting started

1. Get your own legitimate copy of the game. This repository does not provide it.
2. Use Windows 10 or 11 with an NVIDIA card and a recent driver. Keep at least 16 GB of RAM.
3. Build the runtime from a clean checkout of the base project and run it from the folder that holds your copy of the game.
4. The first start of a new build restarts by itself once (warm-up). This is expected.

### Commands and variables used

| Command | What it does |
|---|---|
| `run <game_scarlett_master.exe> --execute` | starts the title from your game folder |
| `run ... --execute --console series-s` | asks the title for the Series S profile (1280x720 in this page's measurements) |
| `cargo test -p victory-shader` | runs the shader translator tests |
| `cargo test -p victory-gpu` | runs the GPU host tests |

Variables used in the best result: `VICTORY_DISPLAY_MODE=1280x720`, `VICTORY_CAPTURE` and `VICTORY_CAPTURE_WORLD=1` (diagnostic). Defaults kept: `VICTORY_COUNT_VIEWS` off, `VICTORY_RAW_VIEWS` not set, `VICTORY_SNAPSHOT_POOL` 1024, `VICTORY_TEXTURE_BUDGET_MB` 2048, MFG off and no downloads. Variables the fork code does not read: `VICTORY_TEXTURE_PASS_MS` and `VICTORY_FILL_WORKERS`; they had no effect.

### Debugging

- **Crash log:** `%LOCALAPPDATA%\victory\data\<title-id>\crashes.log`, with the exception code, the address and the stack.
- **Minidumps:** written for exceptions in the game's code (Windows Error Reporting). Host failures may not produce a dump.
- **Direct3D 12 debug layer:** install "Graphics Tools" (`Add-WindowsCapability -Online -Name 'Tools.Graphics.DirectX~~~~0.0.1.0'`, as administrator, then restart). With `VICTORY_D3D12_DEBUG=1`, a diagnostics build can name the failing command; in our runs, it did not.
- **Debugger:** WinDbg Preview includes `cdb.exe`. Attaching a debugger changed timing in our runs, so results are not directly comparable.

### Documentation

- [Architecture](docs/arquitetura.md)
- [Overview](docs/visao-geral.md)
- [Game file policy](docs/politica-de-arquivos.md)

### How to contribute

Send a tested configuration with the "Test report" issue template; see [CONTRIBUTING.md](CONTRIBUTING.md). Do not send game files, shader caches, executables or keys.

### Credits

The diagnostics and measurements in this investigation are our own. The environment variable ideas and the code base come from Victory and the projects cited in [docs/visao-geral.md](docs/visao-geral.md).

### License

This project is licensed under the GNU General Public License v3.0 or later (see [LICENSE](LICENSE)), the same license as the base project. Any derived work must keep this license and state its changes.