# tcc-ufsm-2020
Este é o repositório do meu trabalho de conclusão de curso (TCC) da UFSM.

Como o trabalho ainda está em sua fase inicial, os detalhes serão descritos
futuramente.

## Execução
Primeiro, [instale e configure o Rust na sua
máquina](https://doc.rust-lang.org/book/ch01-01-installation.html).

Após isso, baixe este repositório e execute no terminal o comando ```cargo run```.

## Main Quest
Ou, _"seria bom se eu fizesse tudo isso!_". Ordem de prioridade, mais ou menos.
- [x] Estruturar o básico do básico do [bracket-lib](https://github.com/thebracket/bracket-lib) 
  (RLTK);
- [x] Movimento do jogador @;
- [x] Estrutura básica de um mapa;
- [x] Arquivo separado para a renderização do mapa e das entidades;
- [x] Sistema de FOV (_field-of-view_);
- [x] Câmera/viewport;
- [x] Implementar uma UI básica e aproveitar para aprimorar os estados de jogo (game states);
- [x] Mobs e estrutura básica do sistema de combate;
- [x] Alguns métodos construtivos de geração de mapas:
    - [x] Random Walkers;
    - [x] Cellular Automata (CA);
        - [x] Assegurar conectividade.
    - [x] BSP (binary space partitioning) dungeons;
    - [x] Diggers/Tunnelers.
        - [x] Retoques finais.
- [x] Sistema de geração de mapas (pipeline) híbrido utilizando
  [WFC](https://github.com/mxgmn/WaveFunctionCollapse) em conjunto com outros algoritmos;
  - [x] Carregar mapa externo desenhado manualmente;
  - [x] Aplicar WFC sobre o mapa atual;
  - [x] Assegurar conectividade pelo método do flood-fill (CA);
  - [ ] Assegurar conectividade por outros métodos;
  - [ ] Melhoramentos (e.g. wrapping), etc. 
- [x] Inserção de estruturas pré-fabricadas no mapa;
- [x] Temáticas diferentes de mapas:
    - [x] TDCL (top-down cavern-like);
    - [x] TDML (top-down mansion-like);
    - [x] Florestas;
    - [x] Ruínas;
    - [ ] Algo envolvendo WFC (importante!!!)
    - [ ] Outros (cidades, canyons, etc.).
- [x] Inventário e consumo de itens;
- [x] Equipamento;
- [ ] Baús de tesouro;
- [ ] Sistema de spawning;
- [ ] Usar [RON](https://github.com/ron-rs/ron) (e não JSON) para estruturar os raws;
- [ ] Sistema de serialização/desserialização usando RON +
  serde para mobs, itens e cores.
- [ ] Combate melhorado (e.g. torná-lo estocástico);
    - [ ] Também aplicar bônus dos equipamentos!
- [ ] Itens únicos com efeitos aleatórios;
- [ ] IA relativamente avançada para os mobs.

Naturalmente, à medida que vou desenvolvendo posso ter de 
alterar/aprimorar itens da checklist já marcados. Isso é um processo natural;
considere que itens marcados já possuem a _estrutura básica_ concluída. 

###  Sidequests
Ou, _"se sobrar tempo eu faço!"_.
- [ ] Narrativa procedural;
- [ ] Sistema de partículas;
- [ ] Otimizações gerais;
- [ ] Refatoração no código da UI (chato);
- [ ] Salvar o jogo (extremamente tedioso).

## Problemas conhecidos, etc.
- Distorção dos tiles dependendo da resolução.
    - TODO: ajustar tamanho do tile de acordo com a resolução do usuário.

## Contribuições
Se você tiver alguma boa ideia ou sugestão, sinta-se livre para abrir um 
[_issue_](https://github.com/pprobst/tcc-ufsm-2020/issues/new).

## Referências e inspirações
Em breve.
