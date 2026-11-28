# Notas: Series S 720p, cena do mundo

## PT

### Configuração real do melhor resultado
- Executável: build de diagnóstico com captura de quadro (run-count\victory-count.exe), comando "run <jogo> --execute --console series-s".
- Perfil: Xbox Series S (--console series-s), configuração de 16 GB, conforme o cabeçalho do log.
- Modo de exibição reportado ao título: 1280x720 (VICTORY_DISPLAY_MODE=1280x720, confirmado no log).
- Contador de views (VICTORY_COUNT_VIEWS): desligado.
- Raw twins das views (VICTORY_RAW_VIEWS): não definido, valor padrão (ligado).
- MFG Unlocker: desligado. Downloads: nenhum.
- Captura de mundo: VICTORY_CAPTURE + VICTORY_CAPTURE_WORLD=1.
- Não usados nessa sessão: VICTORY_SNAPSHOT_POOL (padrão 1024) e VICTORY_TEXTURE_BUDGET_MB (padrão 2048 MB). Não entram na configuração.
- Variáveis que o script define, mas o código do fork não lê: VICTORY_TEXTURE_PASS_MS e VICTORY_FILL_WORKERS. Não tiveram efeito e não são citadas como causa.

### Medição de 10 minutos (uma amostra por minuto)

| min | FPS | ms/quadro | draws/quadro | descartes/quadro | erros de GPU | captura |
|---|---|---|---|---|---|---|
| 0 | 2,1 | 467 | 4358 | 246 | 0 | frame-4445 (922 KB) |
| 1 | 1,7 | 583 | 5670 | 278 | 0 | frame-4589 (905 KB) |
| 2 | 1,4 | 723 | 5738 | 320 | 0 | frame-4669 (911 KB) |
| 3 | 2,3 | 444 | 5951 | 284 | 0 | frame-4765 (928 KB) |
| 4 | 2,3 | 436 | 5947 | 272 | 0 | frame-4909 (933 KB) |
| 5 | 2,5 | 400 | 6014 | 281 | 0 | frame-5053 (924 KB) |
| 6 | 2,3 | 435 | 6021 | 277 | 0 | frame-5189 (943 KB) |
| 7 | 2,4 | 413 | 6187 | 288 | 0 | frame-5325 (946 KB) |
| 8 | 1,8 | 555 | 6180 | 267 | 0 | frame-5453 (952 KB) |
| 9 | 2,1 | 481 | 6194 | 265 | 0 | frame-5573 (944 KB) |
| 10 | 1,7 | 587 | 6204 | 268 | 0 | frame-5693 (936 KB) |

Resultado: sem queda e sem erro de GPU durante os 10 minutos.

### Estado atual
- Sessão em execução desde 23:42 (horário local), processo victory-count (PID 19212).
- Cena: corredor de presídio com a cela A106, personagem de costas (captura frame-8669).
- Antes disso, os quadros de mundo frame-2643 e frame-2669 estavam pretos com borda vermelha. Não há comparação entre quadros da mesma cela.
- Registro por minuto contínuo: logs\registro-19212-continuo.csv.

### Não verificado
- Se a cena ficou na mesma cela durante toda a medição (os quadros não foram comparados entre si).
- Efeito real de VICTORY_SNAPSHOT_POOL e VICTORY_TEXTURE_BUDGET_MB: não foram usados.
- Qualidade de imagem além do quadro observado.

## EN

### Actual configuration of the best result
- Executable: diagnostic build with frame capture (run-count\victory-count.exe), command "run <game> --execute --console series-s".
- Profile: Xbox Series S (--console series-s), 16 GB configuration, as stated in the log header.
- Display mode reported to the title: 1280x720 (VICTORY_DISPLAY_MODE=1280x720, confirmed in the log).
- View counter (VICTORY_COUNT_VIEWS): off.
- View raw twins (VICTORY_RAW_VIEWS): not set, default (on).
- MFG Unlocker: off. Downloads: none.
- World capture: VICTORY_CAPTURE + VICTORY_CAPTURE_WORLD=1.
- Not used in this session: VICTORY_SNAPSHOT_POOL (default 1024) and VICTORY_TEXTURE_BUDGET_MB (default 2048 MB). Not part of the configuration.
- Variables the script sets but the fork code does not read: VICTORY_TEXTURE_PASS_MS and VICTORY_FILL_WORKERS. They had no effect and are not cited as a cause.

### 10-minute measurement (one sample per minute)

Same table as above.

Result: no crash and no GPU error during the 10 minutes.

### Current state
- Session running since 23:42 (local time), process victory-count (PID 19212).
- Scene: prison corridor with cell A106, character seen from behind (capture frame-8669).
- Earlier, world frames frame-2643 and frame-2669 were black with a red border. No comparison between frames of the same cell.
- Continuous per-minute log: logs\registro-19212-continuo.csv.

### Not verified
- Whether the scene stayed in the same cell for the whole measurement (frames were not compared with each other).
- Real effect of VICTORY_SNAPSHOT_POOL and VICTORY_TEXTURE_BUDGET_MB: not used.
- Image quality beyond the observed frame.

## Segunda medição (sessão reaberta, 27 a 28/11)

- A sessão anterior parou de gravar log às 00:13; a sessão reaberta voltou a gravar às 01:47, com o mesmo save e o mesmo cache.
- Última leitura (02:19 local): 2,1 FPS, 470 ms por quadro, 3.609 desenhos por quadro, 424 descartados sem pipeline.
- Tempo por quadro: 110 ms de cópias no preparo, 18 ms de texturas, 79 ms na GPU e 219 ms de cópia de volta.
- Processo vivo e sem queda até 02:19. Ainda não é jogável: a cena observada após o aperto de A é uma tela de introdução ou carregamento, não confirmada como mundo.

## Second measurement (reopened session, 27 to 28/11)

- The previous session stopped writing its log at 00:13; the reopened session resumed writing at 01:47, with the same save and cache.
- Last reading (02:19 local): 2.1 FPS, 470 ms per frame, 3,609 draws per frame, 424 dropped without a pipeline.
- Frame time: 110 ms of copies in preparation, 18 ms of textures, 79 ms on the GPU and 219 ms of copy-back.
- Process alive and no crash up to 02:19. Still not playable: the scene seen after pressing A is an intro or loading screen, not confirmed as the world.