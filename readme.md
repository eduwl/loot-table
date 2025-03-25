## Como Usar

Com o executavel na pasta que desejar abra o terminal no diretório e utiliza da seguinte forma:

- Exemplo

```bash
./loot-table <nome_boss> <iteralções>
```

```bash
./loot-table kaari 1000
```

```bash
 ## Windows
.\loot-table.exe kaari 1000
```

> Utilize com o `stalker`, `deva`, `kaari` ou `profane`.

## Loot Table

Uma **loot table** é uma tabela de probabilidades usada para determinar quais itens são dados ao jogador após realizar uma ação no jogo, como derrotar um inimigo. A tabela contém itens, cada um com uma **probabilidade** de ser escolhido, e a soma das probabilidades de todos os itens é, normalmente, **100%** (ou algo muito próximo disso).

A ideia por trás de uma **loot table** é criar uma seleção aleatória e controlada dos itens que o jogador pode obter, garantindo que itens raros tenham uma chance menor de aparecer e itens comuns tenham uma chance maior.

### Como funciona a loot table em termos práticos

Vamos detalhar o funcionamento do seu código usando um exemplo de **loot table** para o "Kaari":

#### Exemplo de loot table para o "Kaari":

| Item                           | Chance (%) |
| ------------------------------ | ---------- |
| Small Kaari Soul Shield Chest  | 25%        |
| Medium Kaari Soul Shield Chest | 20%        |
| Valor Stone Chest              | 22.8%      |
| Kaari Mask                     | 4%         |
| Kaari Googles                  | 4%         |
| Kaari Suit                     | 4%         |
| Moonwater Plains King Kaari    | 0.2%       |
| Kaari Jiangshi Weapon Chest    | 20%        |

Neste exemplo, a soma das probabilidades é 100%. Quando o jogo for executado, o sistema vai fazer uma "rolagem" para determinar qual item será obtido. Vamos explicar como isso funciona no código:

---

### Passos para gerar o loot

1. **Geração de um número aleatório**:
   Quando a função `roll_loot` é chamada, ela gera um número aleatório (entre 0 e 100) para simular a rolagem de "probabilidade" do loot.

   ```rust
   let roll = rand::thread_rng().gen_range(0.0..100.0);
   ```

2. **Comparando com as probabilidades**:
   O número gerado é então comparado com as probabilidades acumuladas na **loot table**. A soma das probabilidades é acumulada conforme você percorre os itens na tabela.

   Por exemplo:

   - O primeiro item tem 25% de chance.
   - O segundo item tem 20% de chance, então a soma acumulada para o segundo item é 45%.
   - O terceiro item tem 22.8%, então a soma acumulada vai até 67.8%.
   - E assim por diante...

3. **Determinando o item escolhido**:
   Quando a rolagem é feita, o código verifica qual item corresponde ao número gerado. O número gerado deve ser comparado com as probabilidades acumuladas até encontrar a faixa correspondente.

   Se o número gerado for menor que a soma acumulada de 25%, o **Small Kaari Soul Shield Chest** é escolhido. Se for menor que 45%, o **Medium Kaari Soul Shield Chest** será o item sorteado, e assim por diante.

---

### Como a acumulação funciona

Vamos detalhar as **probabilidades acumuladas** de forma mais clara para o exemplo da tabela acima:

| Item                           | Chance (%) | Acumulação (%) |
| ------------------------------ | ---------- | -------------- |
| Small Kaari Soul Shield Chest  | 25%        | 25%            |
| Medium Kaari Soul Shield Chest | 20%        | 45%            |
| Valor Stone Chest              | 22.8%      | 67.8%          |
| Kaari Mask                     | 4%         | 69.8%          |
| Kaari Googles                  | 4%         | 73.8%          |
| Kaari Suit                     | 4%         | 77.8%          |
| Moonwater Plains King Kaari    | 0.2%       | 78%            |
| Kaari Jiangshi Weapon Chest    | 20%        | 98%            |

### Como o código usa essas informações:

1. O código gera um número aleatório (digamos, `roll = 43`).
2. O código então compara esse número com as probabilidades acumuladas:

   - 43 está entre 25 e 45, então o item sorteado é o **Medium Kaari Soul Shield Chest**.

3. A quantidade de vezes que um item é sorteado pode ser acumulada, como mostrado no código com a estrutura `HashMap`. Ele armazena o número de vezes que cada item foi escolhido ao longo de várias iterações.

### Exemplificando o comportamento do código:

- O código irá executar a função `roll_loot` para o boss escolhido (`stalker`, `deva`, etc.) várias vezes, dependendo das **iterações** fornecidas como entrada.
- A cada iteração, o item escolhido é adicionado ao `HashMap`, que acumula a quantidade de vezes que cada item foi sorteado.
- Ao final das iterações, o código imprime a quantidade total de vezes que cada item apareceu.

### Acumulando os resultados:

No código, a função `loot.entry(result.0).or_insert(0) += result.1` é responsável por acumular o número de vezes que um item foi sorteado. Ou seja:

- `result.0` é o nome do item sorteado.
- `result.1` é a quantidade (sempre 1, já que é um sorteio binário, ou seja, um item por vez).
- `or_insert(0)` inicializa a quantidade de um item com 0 caso ainda não tenha sido sorteado, e depois a quantidade é aumentada com `+= result.1`.

---

### Conclusão

- **Loot Table** é uma estrutura de dados usada para armazenar itens e suas probabilidades de serem sorteados.
- O código gera um número aleatório e compara esse número com as probabilidades acumuladas para determinar qual item o jogador vai receber.
- A quantidade de cada item sorteado é acumulada em um `HashMap` e o resultado final é impresso após todas as iterações.

Isso garante que itens mais raros, com menor probabilidade, serão sorteados menos frequentemente, enquanto itens mais comuns, com maior probabilidade, serão sorteados mais vezes.
