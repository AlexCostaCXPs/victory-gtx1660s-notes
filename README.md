# Victory na GTX 1660 SUPER: notas de desempenho

Notas de uma investigação de desempenho do Victory em uma GeForce GTX 1660 SUPER (6 GB), no Windows 11.

**Este repositório não contém o jogo, arquivos de jogo, cache de shaders nem executáveis.** É preciso ter a sua própria cópia.

## Base

Este trabalho parte do Victory publicado em [Marvisak/VIctory](https://github.com/Marvisak/VIctory), sob GPL v3 (ver `LICENSE`). Mudanças e diagnósticos próprios também seguem GPL v3.

Projetos consultados para comparação (não foram portados aqui): [uiliannn/VItoriaBr](https://github.com/uiliannn/VItoriaBr) e [fxceless/victory-fork](https://github.com/fxceless/victory-fork).

## O que foi medido

| Medição | Resultado |
|---|---|
| Passe de texturas com `VICTORY_TEXTURE_PASS_MS=50` (ante 250) | passe caiu de 225–273 ms para 55–57 ms; quadro médio no primeiro minuto da intro caiu de 1.892 para 1.451 ms |
| Cópia de volta de buffers (diagnóstico) | cerca de 81% do tempo da cópia está na comparação dos buffers feita na CPU; mais de 99% dos bytes comparados já eram iguais |
| Candidatos a excluir da comparação | 99,7% dos bytes candidatos estavam cobertos por intervalos que precisam ser comparados; ganho estimado de cerca de 0,005% dos bytes |
| `VICTORY_WIDE_REFRESH=1` | preparo de cópias caiu de 399 para 349 ms, mas o quadro subiu de 1.698 para 2.318 ms |
| `VICTORY_WIDE_MERGE=1` | caiu em cerca de 30 s com 0xC0000409 |
| `VICTORY_FILL_WORKERS=3` com `VICTORY_CARD_BUFFERS_MIN_MB=1` | caiu em cerca de 2,5 min com 0xC0000409 |
| `VICTORY_CARD_BUFFERS_MIN_MB=1` | sem ganho claro; faixa de 1,2–1,5 FPS, cerca de 744 ms por quadro |

## O que ainda não foi resolvido

- O jogo fica perto de 1 FPS na GTX 1660 SUPER.
- Houve quedas com 0xC0000005 (violação de memória) e 0x887A0005 (dispositivo de GPU removido). A causa não foi identificada: o crash log não traz a pilha e o envio ao Backtrace foi bloqueado.
- A captura de minidump local (WER LocalDumps) cobre exceções do código do jogo, não falhas no host.

## Próximos passos

1. Capturar um minidump na próxima queda e identificar o módulo.
2. Medir a comparação na CPU separadamente da espera da GPU.
3. Testar `VICTORY_FILL_WORKERS=1` sem outras flags, para separar a causa das quedas.

## Crédito

Os diagnósticos e medições desta investigação são próprios. Os nomes e as ideias de opções de ambiente vêm do Victory base e dos projetos citados acima.
