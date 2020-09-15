# tcc-ufsm-2020
This is the repository for my bachelor's degree final project (UFSM).

## Wait! I read your article from SBGames 2020! Where do I go?
[/src/map_gen/](https://github.com/pprobst/tcc-ufsm-2020/tree/master/src/map_gen) contains all the algorithms presented in the article. They're
generally easy to follow, but for fully understanding WaveFunctionCollapse it
would be wise to read Gridbugs' [article](https://gridbugs.org/wave-function-collapse/) (my implementation directly derives from it) or Karth & Smith [research paper](https://adamsmith.as/papers/wfc_is_constraint_solving_in_the_wild.pdf).
Keep in mind that the procgen pipelines are still far from perfect, and I'll
still keep working on them. I'm also working on the general game mechanics and
in the future I'll play with procedural narrative -- a new interest for me.

However, this is a game prototype (not a procedural dungeon generation tool!) and still not ready for playing. But if
you know your way around Rust and want to fiddle with the procgen algorithms,
feel free to do it!

If you want to contact me for whatever reason, send me an email. You can use the email on my GitHub
profile or the one in the paper.

## Execution
First, [install and configure Rust in your machine](https://doc.rust-lang.org/book/ch01-01-installation.html).

Then, clone this repository, navigate to it and run ```cargo run``` from your terminal emulator.

Tell me if you have any problems.

---

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
  - [ ] Melhoramentos (e.g. wrapping, ?), etc. 
- [x] Inserção de estruturas pré-fabricadas no mapa;
- [x] Temáticas diferentes de mapas:
    - [x] TDCL (top-down cavern-like);
    - [x] TDML (top-down mansion-like);
    - [x] Florestas;
    - [x] Ruínas;
    - [x] WFC como arquitetura externa/interna.
- [x] Inventário e consumo de itens;
- [x] Equipamento;
- [x] Baús de tesouro;
- [x] Seleção de regiões no mapa para aplicar algoritmos de geração;
- [x] Usar [RON](https://github.com/ron-rs/ron) (e não JSON) para estruturar os raws;
- [x] Sistema de serialização/desserialização básico usando RON +
  serde para mobs, itens e cores.

Naturalmente, à medida que vou desenvolvendo posso ter de 
alterar/aprimorar itens da checklist já marcados. Isso é um processo natural;
considere que itens marcados já possuem a _estrutura básica_ concluída. 

###  Sidequests
Objetivos opcionais. Provavelmente não serão realizados no momento, mas de
qualquer forma serão inseridos no futuro.
- [x] Sistema de spawning;
- [ ] Combate melhorado (e.g. torná-lo estocástico);
    - [ ] Também aplicar bônus dos equipamentos, que por enquanto não fazem
      nada.
- [ ] IA relativamente avançada para os mobs.
- [ ] Narrativa procedural;
- [ ] Itens únicos com efeitos aleatórios;
- [ ] Sistema de partículas;
- [ ] Expandir o log;
- [ ] Refatoração no código da UI;
- [ ] Salvar e carregar o jogo.
- [ ] Narrativa procedural;

## Problemas conhecidos, etc.
- Distorção dos tiles dependendo da resolução.
    - TODO: ajustar tamanho do tile de acordo com a resolução do usuário.

## Contribuições
Se você tiver alguma boa ideia ou sugestão, sinta-se livre para abrir um 
[_issue_](https://github.com/pprobst/tcc-ufsm-2020/issues/new).

## Referências e inspirações
Em breve.
