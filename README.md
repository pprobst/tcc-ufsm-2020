# tcc-ufsm-2020
Este é o repositório do meu trabalho de conclusão de curso (TCC) da UFSM.

Como o trabalho ainda está em sua fase inicial, os detalhes serão descritos
futuramente.

## Execução
Primeiro, [instale e configure o Rust na sua
máquina](https://doc.rust-lang.org/book/ch01-01-installation.html).

Após isso, baixe este repositório e execute no terminal o comando ```cargo run```.

## Main Quest
Ou, _"seria bom se eu fizesse tudo isso!_".
- [x] Estruturar o básico do básico do [bracket-lib](https://github.com/thebracket/bracket-lib) 
  (RLTK);
- [x] Movimento do jogador @;
- [x] Estrutura básica de um mapa;
- [x] Arquivo separado para a renderização do mapa e das entidades;
- [x] Sistema de FOV (_field-of-view_);
- [x] Câmera/viewport;
- [ ] Implementar uma UI básica e aproveitar para aprimorar os estados de jogo (game states);
- [ ] Mobs e combate básico;
- [ ] Equipamento e inventário;
- [ ] Sistema de geração de mapas (pipeline) híbrido utilizando
  [WFC](https://github.com/mxgmn/WaveFunctionCollapse) em conjunto com outros algoritmos;
- [ ] Temáticas diferentes de mapas (ruínas, florestas, aquedutos, etc.);
- [ ] Usar [RON](https://github.com/ron-rs/ron) (e não JSON) para estruturar os raws;
- [ ] Sistema de serialização/desserialização usando RON +
  serde;
- [ ] Itens únicos com efeitos aleatórios;
- [ ] Sistema de IA relativamente avançada para os mobs.

Naturalmente, à medida que vou desenvolvendo posso ter de 
alterar/aprimorar itens da checklist já marcados. Isso é um processo natural;
considere que itens marcados já possuem a _estrutura básica_ concluída. 

###  Sidequests
Ou, _"se sobrar tempo eu faço!"_.
- [ ] Narrativa procedural;
- [ ] Sistema de partículas;
- [ ] Fontes melhores (ttf?);
- [ ] Otimizações gerais;
- [ ] Salvar o jogo (tedioso de programar!).

## Problemas conhecidos
- Alta distorção dos tiles ao usar um tamanho de tile inferior a 16x16.
    - Muito provavelmente tem a ver com a resolução. Por ora, deixei o jogo
      ajustado para a resolução do meu laptop: 1366x768, com tiles de tamanho
      16x16. Não resolve o problema _per se_, mas ao menos não há distorções tão perceptíveis.

## Contribuições
Se você tiver alguma boa ideia ou sugestão, sinta-se livre para abrir um 
[_issue_](https://github.com/pprobst/tcc-ufsm-2020/issues/new).

## Referências e inspirações
Em breve.
