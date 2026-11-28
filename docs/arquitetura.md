# Arquitetura

O jogo de console roda como código x86-64 dentro de um processo do Windows. A camada do VIctory fornece o que o console forneceria:

1. **Carregador:** coloca o executável e as DLLs do título na memória e liga as chamadas do sistema do console às nossas implementações.
2. **CPU:** o código x86 do título roda direto. Instruções que o processador do PC não tem são capturadas quando falham e emuladas.
3. **GPU:** os comandos do Direct3D do console são traduzidos para Direct3D 12 do PC. Os shaders são recompilados para DXIL na primeira vez que são usados. Os dados da memória do console são copiados para a memória da placa e de volta; é aqui que se gasta a maior parte do tempo.
4. **Serviços do sistema:** arquivos, áudio (XMA), entrada, saves e licenças.

Os gargalos medidos estão na comparação de buffers na CPU e no trabalho da GPU por quadro. Os detalhes estão no README.