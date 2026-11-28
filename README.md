# Victory on a GeForce GTX 1660 SUPER: performance and stability notes

[Português](#português) · [English](#english)

## Scope and status

This repository is a **performance investigation** of a Direct3D 12 compatibility layer on NVIDIA GPUs. It is not a build of any game.

- It contains **no game files**: no game executables, no game data, no caches and no game builds. The user supplies their own files, and the layer reads them at run time only.
- The source code is licensed under **GPL-3.0-or-later** (see [LICENSE](LICENSE)). Some files include third-party code under MIT or LGPL terms; their notices are kept in place.

### Progress table

| Stage | Status | Notes |
|---|---|---|
| Preparation (setup, configuration) | Done | Scripts and build instructions are in this repository. |
| Executable running | Done | The title starts under a Series S profile at 1280x720. |
| First frame | Done | Frames are presented; the first ones are black. |
| Main menu | Done | The account selection screen was observed in captures. |
| In the world | Partial | Over 900 draws per frame were reached in a logged session. Early world frames were black. |
| Playable | Not verified | No session with input beyond the menu was checked. |
| Story | Not verified | Not tested. |
| Graphics | Partial | One world frame (a prison corridor) looked correct. Frames were not compared with each other. |
| Performance | Measured | About 2 FPS (1.4 to 2.5) in a 10-minute measurement at 1280x720, with 4,400 to 6,200 draws per frame. |
| Stability | Partial | No crash and no GPU error in the first 10 minutes. Afterwards the game kept rendering, but its log stopped being written, so later GPU errors could not be checked. |

### Commands and settings actually used

- Command: `victory run <game> --execute --console series-s` (diagnostic build with frame capture).
- Settings in effect: `VICTORY_DISPLAY_MODE=1280x720`, `VICTORY_CAPTURE=<folder>`, `VICTORY_CAPTURE_WORLD=1`, cache and data folders in the user's own directories.
- Defaults kept: `VICTORY_COUNT_VIEWS` off, `VICTORY_RAW_VIEWS` not set (on), `VICTORY_SNAPSHOT_POOL` (1024), `VICTORY_TEXTURE_BUDGET_MB` (2048 MB), MFG off, no downloads.
- Set by the script but not read by this code: `VICTORY_TEXTURE_PASS_MS` and `VICTORY_FILL_WORKERS`. They had no effect.

Earlier sessions had GPU driver faults and out-of-memory errors in command lists. Those sessions are not part of the results above.

### Situação em português

Investigação de desempenho de uma camada de compatibilidade Direct3D 12 em GPUs NVIDIA. Não é build de nenhum jogo: não há arquivos, executáveis, dados, caches nem builds de jogo no repositório. O código é GPL-3.0-or-later (ver [LICENSE](LICENSE)); avisos de código de terceiros (MIT e LGPL) foram mantidos.

Resultado medido: Series S em 1280x720, imagem de mundo correta em um quadro (corredor de presídio), cerca de 2 FPS (1,4 a 2,5) em 10 minutos, sem queda nem erro de GPU nesse período. Jogável e história não verificados. Depois dos primeiros 10 minutos, o processo seguiu respondendo, mas o log parou de ser escrito, então erros posteriores não foram verificados.


**Para quem quer continuar o trabalho:** veja [CONTRIBUTING.md](CONTRIBUTING.md) para enviar uma configuração testada (há um modelo de issue "Relato de teste").

---

## Português

Notas de uma investigação de desempenho e estabilidade do Victory (runtime que roda o código de um jogo de console x86-64 num PC com Windows) em uma GeForce GTX 1660 SUPER (6 GB de VRAM), com 16 GB de RAM, no Windows 11.

**Este repositório não contém o jogo, arquivos de jogo, cache de shaders, executáveis ou logs com caminhos pessoais.** É preciso ter a sua própria cópia legítima do jogo.

### Resumo

- **Resultado atual:** no perfil Series S em 1280x720, o jogo chegou ao mundo, com imagem boa nesta sessão, e ficou estável por mais de 10 minutos, a cerca de 2 FPS (1,4 a 2,5). Ainda não é jogável: não testamos entradas além do menu, e as primeiras capturas de mundo saíram pretas.
- **Causa principal observada nas sessões anteriores:** falta de memória da placa. O título pedia mais memória de textura do que os 6 GB de VRAM aguentavam, e uma operação do driver falhava (`0x8007000E`). Com o limite de texturas em 2048 MB e o perfil Series S, essa falha não apareceu nos últimos testes.
- **Estado atual:** o jogo chegou ao mundo com imagem boa, no perfil Series S em 1280x720, e ficou estável por mais de 10 minutos, a cerca de 2 FPS. Antes, a decisão era parar; essa decisão foi revista, e a medição continua. Os detalhes estão abaixo.

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

- No perfil Series S em 1280x720, o jogo fica em cerca de 2 FPS (1,4 a 2,5 em 10 minutos), com 4.400 a 6.200 desenhos por quadro. Ainda não testamos jogabilidade além do menu.
- Não há símbolos da imagem do título, então a função que falha não foi identificada.
- A captura de minidump local cobre exceções do código do jogo, não falhas no host.
- Os números vêm de sessões curtas; não há medições em outra máquina.
- A tradução do menu F1 para português existe em uma cópia de trabalho e funciona (o menu abre em português). O código dela não foi publicado aqui.

### Por que paramos (decisão anterior, revista)

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
## Português: primeiros passos, comandos, validador, depuração e licença

### Primeiros passos

1. Tenha a sua própria cópia legítima do jogo. Este repositório não a fornece.
2. Use Windows 10 ou 11 com uma placa NVIDIA e um driver recente. Tenha pelo menos 16 GB de RAM; o fork de referência pede 32 GB.
3. Compile o runtime a partir de um checkout limpo do projeto base (ver Crédito) e rode a partir da pasta com a sua cópia do jogo, com `run <game_scarlett_master.exe> --execute`.
4. O primeiro início de uma build nova reinicia sozinho uma vez (aquecimento). Isso é esperado.

### Comandos

| Comando | O que faz |
|---|---|
| `run <game_scarlett_master.exe> --execute` | inicia o título a partir da pasta do seu jogo |
| `run ... --execute --console series-s` | pede ao título o perfil Series S (1920x1080 nos nossos testes) |
| `play` (subcomando do fork) | inicia com o lançador do fork e o menu de configurações |
| `victory.exe get-d3d12` | baixa o runtime D3D12 da Microsoft, quando o fork oferece |
| `victory.exe get-mfg` | baixa um mod de terceiros opcional (desligado por padrão; não recomendado) |
| `cargo test -p victory-shader` | roda os testes do tradutor de shaders |
| `cargo test -p victory-gpu` | roda os testes do host de GPU |

Variáveis de ambiente úteis (todas opcionais): `VICTORY_TEXTURE_PASS_MS=50` (único ganho confirmado), `VICTORY_FILL_WORKERS=1`, `VICTORY_CARD_BUFFERS_MB`, `VICTORY_SNAPSHOT_POOL`, `VICTORY_TEXTURE_BUDGET_MB`, `VICTORY_COUNT_VIEWS` (só diagnóstico; precisa de `VICTORY_RAW_VIEWS=0`).

### Validador de shader no Windows

Os testes do tradutor de shaders validam o DXIL gerado com o `dxil.dll` do Windows SDK (Windows Kits 10, x64). Se a DLL não for encontrada, os testes de validação são pulados; instale o Windows SDK para rodá-los por completo. Um teste que passa sem o validador não prova que o shader é válido.

### Depuração

- **Crash log:** `%LOCALAPPDATA%\victory\data\<title-id>\crashes.log` traz o código da exceção, o endereço da falha e a pilha, com módulo e deslocamento. Sem símbolos, não há nomes de função.
- **Minidumps:** um dump local é gravado para exceções no código do jogo (Windows Error Reporting, LocalDumps). Falhas no host podem não gerar dump.
- **Camada de depuração do Direct3D 12:** instale "Ferramentas de Gráficos" (`Add-WindowsCapability -Online -Name 'Tools.Graphics.DirectX~~~~0.0.1.0'`, como administrador, e reinicie). Com `VICTORY_D3D12_DEBUG=1`, um build de diagnóstico nomeia o comando que falha; nas nossas execuções, isso não apareceu.
- **Depurador:** o WinDbg Preview (Microsoft Store) traz o `cdb.exe` dentro da pasta do pacote. Anexar um depurador mudou o tempo de execução nos nossos testes, então os resultados não são diretamente comparáveis.

### Licença

Este projeto é licenciado sob a GNU General Public License v3.0 (ver [LICENSE](LICENSE)), a mesma licença do projeto base. Qualquer trabalho derivado precisa manter essa licença e declarar as mudanças.

## English

Notes from a performance and stability investigation of Victory (a runtime that runs an x86-64 console game on Windows) on a GeForce GTX 1660 SUPER (6 GB VRAM), 16 GB RAM, Windows 11.

**This repository contains no game, no game files, no shader cache, no executables and no logs with personal paths.** You need your own legitimate copy of the game.

### Summary

- **Current result:** on the Series S profile at 1280x720, the game reached the world, with a good picture in this session, and stayed stable for more than 10 minutes, at about 2 FPS (1.4 to 2.5). It is not playable yet: no input beyond the menu was tested, and the first world captures came out black.
- **Main cause observed in earlier sessions:** GPU memory shortage. The title asked for more texture memory than 6 GB of VRAM could hold, and a driver operation failed (`0x8007000E`). With the texture limit at 2048 MB and the Series S profile, this failure did not show up in the latest tests.
- **Current status:** the game reached the world with a good picture, on the Series S profile at 1280x720, and stayed stable for more than 10 minutes, at about 2 FPS. Earlier the decision was to stop; that decision was revised, and measurement continues. Details are below.

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

- On the Series S profile at 1280x720, the game runs at about 2 FPS (1.4 to 2.5 over 10 minutes), with 4,400 to 6,200 draws per frame. Gameplay beyond the menu has not been tested yet.
- There are no symbols for the title image, so the failing function was not identified.
- The local minidump capture covers exceptions in the game's code, not host failures.
- The numbers come from short sessions; there are no measurements on another machine.
- A Portuguese translation of the F1 menu exists in a working copy and works (the menu opens in Portuguese). Its code is not published here.

### Why we stopped (earlier decision, revised)

- System RAM (16 GB) is below what the fork asks for (32 GB).
- 6 GB of VRAM does not hold the textures the title asks for, even with copy limits.
- Image and performance tests stayed incomplete, and the crashes continued from both observed causes (title code access and card memory shortage).

### What I would do differently on another machine

1. Raise RAM to 32 GB and use a card with more VRAM (8 GB or more).
2. Repeat the investigation with symbols for the title image, or with a debugger set up from the start.
3. Only then measure performance, with a fixed scene and frame capture.

### Credits

The diagnostics and measurements in this investigation are our own. The environment option ideas and the code base come from Victory and the projects listed above.

---

## Getting started

1. Get your own legitimate copy of the game. This repository does not provide it.
2. Use Windows 10 or 11 with an NVIDIA card and a recent driver. Keep at least 16 GB of RAM; the upstream fork asks for 32 GB.
3. Build the runtime from a clean checkout of the base project (see Credits), then run it from the folder that holds your copy of the game, with `run <game_scarlett_master.exe> --execute`.
4. The first start of a new build restarts by itself once (warm-up). This is expected.

## Commands

| Command | What it does |
|---|---|
| `run <game_scarlett_master.exe> --execute` | starts the title from your game folder |
| `run ... --execute --console series-s` | asks the title for the Series S profile (1920x1080 in our tests) |
| `play` (fork subcommand) | starts with the fork's launcher and settings menu |
| `victory.exe get-d3d12` | downloads Microsoft's D3D12 runtime, when the fork offers it |
| `victory.exe get-mfg` | downloads an optional third-party mod (off by default; not recommended) |
| `cargo test -p victory-shader` | runs the shader translator tests |
| `cargo test -p victory-gpu` | runs the GPU host tests |

Useful environment variables (all optional): `VICTORY_TEXTURE_PASS_MS=50` (only confirmed gain), `VICTORY_FILL_WORKERS=1`, `VICTORY_CARD_BUFFERS_MB`, `VICTORY_SNAPSHOT_POOL`, `VICTORY_TEXTURE_BUDGET_MB`, `VICTORY_COUNT_VIEWS` (diagnostic only; needs `VICTORY_RAW_VIEWS=0`).

## Windows shader-test validator

The shader translator tests validate the generated DXIL with `dxil.dll` from the Windows SDK (Windows Kits 10, x64). If the DLL is not found, the validation tests are skipped, so install the Windows SDK to run them fully. A test that passes without the validator does not prove the shader is valid.

## Debugging

- **Crash log:** `%LOCALAPPDATA%\victory\data\<title-id>\crashes.log` has the exception code, the faulting address and the stack, with module and offset. Without symbols, names of functions are not available.
- **Minidumps:** a local dump is written for exceptions in the game's code (Windows Error Reporting, LocalDumps). Host failures may not produce a dump.
- **Direct3D 12 debug layer:** install "Graphics Tools" (`Add-WindowsCapability -Online -Name 'Tools.Graphics.DirectX~~~~0.0.1.0'`, administrator, then restart). With `VICTORY_D3D12_DEBUG=1` a diagnostics build names the failing command; in our runs it did not.
- **Debugger:** WinDbg Preview (Microsoft Store) includes `cdb.exe` under its package folder. Note that attaching a debugger changed timing in our runs, so results are not directly comparable.

## License

This project is licensed under the GNU General Public License v3.0 (see [LICENSE](LICENSE)), the same license as the base project. Any derived work must keep this license and state its changes.

---
