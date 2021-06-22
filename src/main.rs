use std::usize;

#[derive(Debug)]
struct StackNode<T> {
    data: T,
    next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { top: None }
    }

    fn push(&mut self, data: T) {
        let mut node = StackNode { data, next: None };
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let node = self.top.take();
        match node {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.data)
            }
        }
    }
}

#[derive(Debug)]
struct Queue<T> {
    qdata: Vec<T>,
    capacity: usize,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            qdata: Vec::with_capacity(size),
            capacity: size,
        }
    }

    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.qdata.len() == self.capacity {
            return Err("No space in queue".to_string());
        }
        self.qdata.push(item);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        let size = self.qdata.len();
        if size > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.qdata.len()
    }
}

fn main() {
    let mut q = Queue::new(2);
    q.enqueue(1).unwrap();
    q.enqueue(2).unwrap();

    if let Err(error) = q.enqueue(3) {
        println!("enqueu error: {}", error);
    }

    println!("size: {}", q.size());
    println!("q: {:#?}", q);

    for _ in 0..3 {
        if let Some(data) = q.dequeue() {
            println!("data = {}", data);
        } else {
            println!("queue empty");
        }
    }

    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("after push, s = {:#?}", s);

    s.pop();
    println!("after pop, s = {:#?}", s);
}
