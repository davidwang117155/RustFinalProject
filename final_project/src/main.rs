#![allow(unused)]
use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    node_list: Vec<GraphNode>,
}

type Link = Option<Rc<RefCell<Box<GraphNode>>>>;

trait Node {
    // fn new() -> GraphNode;
    // the data is the name of the node (e.g. Node 1, Node 2, etc)
    fn data(&mut self) -> u32;
    // the i32 is the weight of the edge
    fn connections(&mut self) -> &mut Vec<(Link,i32)>;

    fn new_connection(&mut self, id: u32, cost: i32) -> &mut Vec<(Link,i32)>;
}

#[derive(Debug)]
pub struct GraphNode {
  data: u32,
  connections: Vec<(Link,i32)>,
}

impl Node for GraphNode {
  // fn new() -> Self {
  //   GraphNode { 
  //     data: 0, 
  //     connections: Vec::new() 
  //   }
  // }
  
  fn data(&mut self) -> u32 {
    self.data
    // unimplemented!()
  }

  fn connections(&mut self) -> &mut Vec<(Link, i32)> {
    &mut self.connections
    // unimplemented!()
  }

  fn new_connection(&mut self, end_id: u32, cost: i32) -> &mut Vec<(Link,i32)> {
    let mut new_node = Rc::new(RefCell::new(Box::new(GraphNode {
      data: end_id,
      connections: Vec::new(),
    })));

    self.connections().push((Some(new_node),cost));

    &mut self.connections
    // unimplemented!()
  }
}

impl Graph {
  pub fn new() -> Self {
    Graph {
      node_list: Vec::new(),
    }
  }
  
  pub fn node_list(&mut self) -> &mut Vec<GraphNode> {
    &mut self.node_list
  }

  pub fn add_node(&mut self, id: u32) -> () {
    let mut new_node = GraphNode {
      data: id,
      connections: Vec::new(),
    };
    self.node_list().push(new_node);
  }

  pub fn add_connection(&mut self, start: u32, end: u32, cost: i32) {
    let mut start_present = false;
    let mut end_present = false;
    for node in self.node_list().into_iter() {
      if node.data() == start {
        start_present = true;
      }
      if node.data() == end {
        end_present = true;
      }
    }

    if start_present && end_present {
      for node in self.node_list().into_iter() {
        if node.data() == start {
          node.new_connection(end, cost);
        }
      }
    } else if !start_present {
      panic!("start node not in list")
    } else {
      panic!("end node not in list")
    }
  }

  pub fn adj_matrix(&mut self) {
    let size = self.node_list().len();
    let prefilled = Array2D::filled_with(42, 2, 3);
  }
}

fn main() {
    println!("Hello, world!");
    let mut graph = Graph::new();
    // println!("New graph created: {:?}", graph);
    graph.add_node(0);
    // println!("First node added: {:?}", graph);
    graph.add_node(1);
    // println!("Second node added: {:?}", graph);
    graph.add_connection(0,1,14);
    println!("Node0 and Node1 connected: {:?}", graph);
    graph.add_connection(1,0,12);
    println!("Node1 and Node0 connected: {:?}", graph);
    // let mut test_node = GraphNode::new();
    // println!("New node created: {:?}", test_node)
}