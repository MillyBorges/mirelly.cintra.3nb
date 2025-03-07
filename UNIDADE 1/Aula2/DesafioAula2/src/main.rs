

/// Um nó da fila.
/// Cada nó contém um elemento e uma referência para o próximo nó.
pub struct Node<T> {
    // O elemento armazenado no nó.
    elem: T,
    // Ponteiro para o próximo nó.
    next: Option<Box<Node<T>>>,
}

/// A fila (queue) propriamente dita.
/// Mantém um ponteiro para o primeiro nó (head) e para o último nó (tail).
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> Queue<T> {
    /// Cria uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Adiciona um novo elemento no final da fila.
    pub fn enqueue(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            elem,
            next: None,
        });

        let new_node_ptr: *mut Node<T> = &mut *new_node;

        if let Some(tail_ptr) = self.tail {
            unsafe {
                (*tail_ptr).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }

        self.tail = Some(new_node_ptr);
        self.length += 1;
    }

    /// Remove e retorna o elemento da cabeça da fila.
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;

            if self.head.is_none() {
                self.tail = None;
            }

            self.length -= 1;
            boxed_node.elem
        })
    }

    /// Retorna uma referência ao elemento da frente da fila sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    /// Retorna o número atual de elementos na fila.
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.dequeue() {}
    }
}

// Esse código será compilado e executado somente ao testar.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() { // Traz para o módulo as definições do escopo anterior para o teste.
    // - "let mut queue = Queue::new();"
    // - "let" declara uma nova variável.
    // - "mut" indica que essa variável pode ter seu valor alterado.
    // - "Queue::new()" chama o método associado "new" para criar uma nova instância da nossa fila.
        let mut queue = Queue::new();
    
    // Adiciona (enqueue) elementos na fila.
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
    
    //Como a Fila é FIFO, o primeiro que entra é o primeiro que sai.
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    // Quando a fila estiver vazia, dequeue deve retornar None.
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peek() {
    // Inicializamos uma fila vazia.
        let mut queue = Queue::new();

    // Adicionamos dois elementos à fila: 1 e 2.
    // O método `enqueue` insere cada elemento no final da fila.
        queue.enqueue(1);
        queue.enqueue(2);

    // Verificamos o elemento da frente da fila sem removê-lo.
    // O método `peek` deve retornar uma referência ao primeiro elemento, que é 1.
    // Usamos `Some(&1)` para indicar que esperamos um valor `Option` contendo uma referência ao valor `1`.
        assert_eq!(queue.peek(), Some(&1));

    // Removemos o elemento da frente da fila.
    // O método `dequeue` deve retornar o elemento removido, que é 1.
    // Após essa operação, o elemento 1 não está mais na fila.
        assert_eq!(queue.dequeue(), Some(1));

    // Verificamos novamente o elemento da frente da fila.
    // Agora, o primeiro elemento é 2, pois o 1 foi removido na operação anterior.
    // `peek` deve retornar uma referência ao novo primeiro elemento, que é 2.
        assert_eq!(queue.peek(), Some(&2));
    }

    #[test]
    fn test_len() {
    // Cria uma nova fila vazia.
        let mut queue = Queue::new();

    // Verifica se o comprimento da fila é 0, pois ela acabou de ser criada e não contém elementos.
        assert_eq!(queue.len(), 0);

    // Adiciona o primeiro elemento (1) à fila.
        queue.enqueue(1);

    // Verifica se o comprimento da fila foi atualizado para 1, refletindo a adição do primeiro elemento.
        assert_eq!(queue.len(), 1);

    // Adiciona o segundo elemento (2) à fila.
        queue.enqueue(2);
    
    // Verifica se o comprimento da fila foi atualizado para 2, refletindo a adição de mais um elemento.
        assert_eq!(queue.len(), 2);

    // Remove o primeiro elemento da fila.
        queue.dequeue();
    
    // Verifica se o comprimento da fila foi atualizado para 1, refletindo a remoção de um elemento.
        assert_eq!(queue.len(), 1);


    // Remove o segundo elemento da fila, que agora é o primeiro após a remoção anterior.
        queue.dequeue();

    // Verifica se o comprimento da fila foi atualizado para 0, refletindo a remoção do último elemento.
        assert_eq!(queue.len(), 0);
    }
}
