# tcc-ufsm-2020
**(EN-US)** This is the repository for my bachelor's degree final project at [UFSM](https://www.ufsm.br/): a game
prototype made to explore the Entity-Component-System (ECS) architecture and procedural dungeon generation.
While the game is not fully playable (or even enjoyable) at the moment, it
fulfills my expectations. A more profound development will occur independently in another
repository, after my graduation.

**(PT-BR)** Este é o repositório do meu trabalho de conclusão de curso (TCC) na [UFSM](https://www.ufsm.br/): um
protótipo de jogo elaborado para explorar a arquitetura *Entity-Component-System* (ECS) e
geração procedural de *dungeons*.
Embora o jogo não seja totalmente jogável (ou mesmo divertido) no momento,
ele cumpre minhas expectativas. Um desenvolvimento mais aprofundado ocorrerá de forma independente
em outro repositório, após a minha graduação.

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

While in-game:
- Use the Vi-keys to move or select targets.
- Space for contextual action (e.g. open doors).
- 'i' to access inventory.
- 'e' to access equipment.
- 'z' to switch between melee/ranged weapons.
- 'f' to target and fire.
- 'r' to reload.

---

## Objetivos principais (TCC)
Ou, _"seria bom se eu fizesse tudo isso!_". Ordem de prioridade, mais ou menos.
- [x] Estruturar o básico do básico do [bracket-lib](https://github.com/thebracket/bracket-lib) 
  (RLTK) + [specs](https://github.com/amethyst/specs/), utilizando o tutorial desenvolvido por Wolverson como base;
- [x] Movimento do jogador @;
- [x] Estrutura básica de um mapa;
- [x] Arquivo separado para a renderização do mapa e das entidades;
- [x] Sistema de FOV (_field-of-view_);
- [x] Câmera/viewport (divergências começam aqui);
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

## Problemas conhecidos, etc.
- O sistema de spawning está longe do adequado.
- Distorção dos tiles dependendo da resolução.
- Alguns crashes ocorrem de vez em quando na etapa de geração de mapas.
    - Causa provável: acesso ao índice 0 do mapa (não utilizado).
- Por enquanto, o WFC não reinicia quando há contradição (raro de acontecer). 

## Contribuições
Se você tiver alguma boa ideia ou sugestão, sinta-se livre para abrir um 
[_issue_](https://github.com/pprobst/tcc-ufsm-2020/issues/new).

## Referências e inspirações
Veja o arquivo da minha monografia para a lista de referências. Outros projetos do GitHub que tiveram
porções de código utilizadas (ou que serviram de base para algo) estão referenciados em forma de comentários nos arquivos de código relevantes.
