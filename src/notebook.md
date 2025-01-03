##Big O: 
    Se refere ao quanto o tamanho do input impacta no tempo e recursos que o programa demanda pra resolver o problema. Ex: se é preciso encontrar um único numero em um array, de input, será O(1) ó de 1, pois precisará percorrer o array apenas uma vez, mas caso por exemplo precise encontrar todos os elementos de um array inputado pelo usuario, será O(n) ó de n, pois precisará percorrer n vezes o array, até encontrar todos os elementos do input do usuário, o tempo variará conforme o tamanho do array inserido pelo usuário.

    O(1) - Ó de 1:
        Independente do tamanho do input, o tempo de execução sempre será o mesmo. É o melhor caso possível.

    O(log n) - Ó log de n:
        O tempo de execução variará de acordo com o input do usuário, mas não de forma linear, e sim de forma logaritimica

    O(n) - Ó de n:
        O tempo de execução será proporcional ao input, se o input for o dobrado, tempo de execução também será dobrado. 

    O(n log n) Ó:
        Parte do algorítmo tem tempo de execução escalado com O(n) e outra parte com O(log n)
    
    O(n²):
        Tempo de execução escala o quadrado do input, ou seja, input de 10, demora 10x10=100, e de 100, demorará 100x100=10000. Isso ocorre com loops dentro de loops.

    O(2^n):
        Tempo exponencial, tempo escala dobrando o tempo maginal a cada unidade marginal adicional no input
    O(n!):
        Tempo fatorial, tempo de execucao aumenta fatorialmente de acordo com o input
    


##Array:    
    Um array é um espaço alocado na memoria, com numero fixo de elementos, sendo um espaço continuo, sem itens no meio (por isso array é imutavel, pois pode haver itens a direita ou a esquerda do array, a solução é criar um novo array e apontar pro novo)
    1



##Linked List:
    Um grupo de elementos conectados, mas não necessariamente um ao lado do outro na memoria, como o array. Caso necessário encontrar item especifico, é preciso percorrer a linked list toda até achar o item, ou seja, O(n), diferente de um array, que é O(1) pois sabendo a posição inicial do array, já se sabe onde está o item. Por outro lado, quando é necessário inserir um item, no caso do array, é necessário encontrar um novo gap livre na memoria que caiba o novo array e mover todos os elementos pro novo local, O(n), enquando em uma linked list é um pouco diferente, pois, é conhecido o head e a tail da linked list, ou seja, um novo elemento no final ou no inicio é O(1), pois ja sabemos a posição, do contrário, se for entre dois elementos dentro da linked list, a operação se torna 0(n) pois é preciso percorrer toda a linked list primeiro, pra depois executar a inserção. Pra deletar um item fuunciona da mesma forma.



##Queue:
    FIFO(first in first out), grupo de elementos travados 



##HashMap:
    Um dicionário de chave valor. Mas pra acessar os itens em O(1) precisa ser um array... Então uma hash function que vai transformar a chave no slot do array que se encontra o valor. No slot estará armazenado a k/v. em geral, é O(1), mas pode virar O(n) caso tenham muitos itens e colisoes.

##Stack:
    Pilha de dados, o último é sempre o primeiro a sair LIFO (last in first out) e implementa put e pop.



##Binary Tree:
    Junção de Linked lists sempre gerando dois nodos por nodo, não é o unico tipo de tree que existe.


##Grafos:
    Usados pra descobrir a menor distancia entre pontos, 