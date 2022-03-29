/// Rust提供了运行时的动态检查，来满足特殊场景下的需求（一个值有多个所有者的情况）
/// Rust使用引用计数的智能指针：Rc（Reference counter）和 Arc（Atomic reference counter）
use std::rc::Rc;

pub fn test_rc() {
    // 当对一个Rc结构进行clone()，不会将其内部的数据复制，只是增加引用计数
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    // 当一个Rc结构离开作用域被drop(),首先只会减少其引用计数，直到为0时，才会真正释放对应的内存
}

/// Rust 必须提供一种机制，让代码可以像 C/C++ 那样，创建不受栈内存控制的堆内存，
/// 从而绕过编译时的所有权规则。Rust 提供的方式是 Box::leak()
/// 实现DAG
#[derive(Debug)]
struct Node {
  id: usize,
  downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
      Self {
        id,
        downstream: None,
      }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
      self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>>{
      self.downstream.as_ref().map(|v| v.clone())
    }
}

pub fn test_dag(){
  let mut node1 = Node::new(1);
  let mut node2 = Node::new(2);
  let mut node3 = Node::new(3);
  let node4 = Node::new(4);

  node3.update_downstream(Rc::new(node4));
  node1.update_downstream(Rc::new(node3));
  node2.update_downstream(node1.get_downstream().unwrap());

  println!("node1: {:?}, node2: {:?}", node1, node2);
}