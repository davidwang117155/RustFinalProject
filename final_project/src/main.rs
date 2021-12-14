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
    let mut matrix = vec![vec![0; size]; size];
    let mut count = 0;
    for node in self.node_list() {
      for end in &node.connections {
        let (y, cost) = end;
        let y_coord = y.as_ref().unwrap().borrow().data;
        // println!("{:?}", y_coord);
        matrix[count as usize][y_coord as usize] = 1;
      }
      count += 1;
      // println!("{:?}", node.connections());
    }

    println!("{:?}", matrix);
  }

  pub fn dijkstras(&mut self, start: u32) {
    let mut start_exist = false;
    let mut dscore = 0;
    let node_list_len = self.node_list.len();
    let mut dscore_vec = vec![i32::MAX;node_list_len];
    // let mut curr_node: GraphNode;
    let mut curr_node = GraphNode {
      data: 0,
      connections: Vec::new(),
    };

    for node in self.node_list() {
      if node.data() == start {
        start_exist = true;
        curr_node = node;
      }
    }

    if !start_exist {
      panic!("node does not exist in tree")
    } else {
      let mut visited_vec = Vec::<u32>::new();
      while visited_vec.len() != node_list_len {
        visited_vec.push(curr_node.data());
        let mut next_node: GraphNode;
        for edge in curr_node.connections {
          let (end, cost) = edge;
          let mut lowest_cost = i32::MAX;
          let end_node = end.as_ref().unwrap().borrow().data;
          if !visited_vec.contains(&end_node) {
            if dscore_vec[end_node as usize] > dscore + cost {
              dscore_vec[end_node as usize] = dscore + cost;
            }
            if cost < lowest_cost {
              lowest_cost = cost;
              // next node to visit is determined by least-cost edge from current node
              next_node = Box::into_inner((*end.as_ref().unwrap().borrow()));
            }
          }
          dscore += lowest_cost;
        }
        curr_node = next_node;
      }

      println!("{:?}", visited_vec);
    }

    println!("{:?}", dscore_vec);
  }
}

fn main() {
    // println!("Hello, world!");
    let mut graph = Graph::new();
    // println!("New graph created: {:?}", graph);
    graph.add_node(0);
    // println!("First node added: {:?}", graph);
    graph.add_node(1);
    // println!("Second node added: {:?}", graph);
    graph.add_connection(0,1,14);
    // graph.add_connection(0,1,15);
    // println!("Node0 and Node1 connected: {:?}", graph);
    graph.add_connection(1,0,12);
    graph.adj_matrix();
    // println!("Node1 and Node0 connected: {:?}", graph);
    // let mut test_node = GraphNode::new();
    // println!("New node created: {:?}", test_node)
}
