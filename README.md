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

## Outros testes e o que não funcionou

| Teste | Resultado |
|---|---|
| Perfil de console Series S (`--console series-s`) | resolução de apresentação foi 1920x1080, não 720p; tela continuou sem imagem completa |
| `VICTORY_DISPLAY_MODE=1280x720` | não muda a resolução de renderização; o título apresentou em 3840x2160 |
| `VICTORY_KEEP_SNAPSHOTS=1` com 256 MB | sem ganho visível; a RAM do sistema chegou a 7 MB livres |
| Ray tracing (`VICTORY_BVH_BUILDER=1`) | não testado por causa do risco de travar a placa |
| Patch 0045 do Kelvaroq (espera de observação) | não se aplica sem portar o 0044 antes; deixado de fora |
| Correção da issue #721 (escrever só bytes alterados) | mais lenta na bancada quando há muitas alterações |
| Função `differing` alternativa (blocos de 1 MiB) | igual com poucas alterações, mais lenta com muitas |

## Quedas e causas investigadas

- **0xC0000005 em `victory-image-…dll+0x23CF683`:** acesso a endereço nulo dentro do código do título. Apareceu em duas execuções. Sem símbolos, não há nome de função.
- **0x80000003 em `victory-image-…dll+0x5149aec`:** parada numa cadeia de chamadas do título. Nosso código tem um ponto de parada, mas só dispara em imports inexistentes, e isso não ocorreu. A mensagem de assert do título não aparece no log.
- **0x887A0005 (dispositivo de GPU removido):** ocorreu logo depois de erros de gravação do DirectStorage (`destination's physical pages could not be written`) e de novos pipelines de shader. Em uma execução, a queda veio cerca de 85 segundos depois dos erros de gravação.
- A camada de depuração do Direct3D 12 foi instalada, mas nenhuma execução mostrou o comando que falhou.

## O que ainda não foi resolvido

- O jogo fica perto de 1 FPS na GTX 1660 SUPER, com a cena do mundo montando, mas a imagem completa não aparece.
- Não há símbolos da imagem do título, então a função que falha não foi identificada.
- A captura de minidump local cobre exceções do código do jogo, não falhas no host.

## Próximos passos

1. Obter símbolos ou uma captura com a mensagem de depuração do título, para identificar o assert de 0x80000003.
2. Relacionar a perda de dispositivo com o carregamento de pipelines novos, num teste isolado.
3. Só depois voltar às medições de desempenho.

## Crédito

Os diagnósticos e medições desta investigação são próprios. Os nomes e as ideias de opções de ambiente vêm do Victory base e dos projetos citados acima.
