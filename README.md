# tcc-ufsm-2020
Este é o repositório do meu trabalho de conclusão de curso (TCC) da UFSM.

Como o trabalho ainda está em sua fase inicial, os detalhes serão descritos
futuramente.

Note que este TCC se difere da maioria por uma miríade de motivos. Além das
linhas de código, está o fato de que eu apenas tenho interesse (e me divirto) em
aprender sobre como fazer jogos e geração procedural, que são interesses
recentes (desde quando comecei a jogar [_Caves of
Qud_](https://store.steampowered.com/app/333640/Caves_of_Qud/) em 2019); também
pretendo continuar a trabalhar neste jogo após a conclusão do TCC, já que nem
50% do que tenho como visão estará implementado. Este trabalho também utiliza
de várias referências da indústria e outras que achei "in the wild", além de
referências propriamente acadêmicas.

Ou seja, é um TCC muito menos "científico" do que a maioria, podendo ser
considerado mais como um _passion project_ meu que apenas estou aproveitando
como trabalho. De qualquer forma, eu absolutamente discordo de que haja apenas
um método de fazer ciência, muito menos acredito cegamente no método científico.

## Execução
Primeiro, [instale e configure o Rust na sua
máquina](https://doc.rust-lang.org/book/ch01-01-installation.html).

Após isso, baixe este repositório e execute no terminal o comando ```cargo run```.

Avise-me caso houver algum problema.

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
