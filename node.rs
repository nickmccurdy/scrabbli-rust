/*
  Value Format:
  - 0 to 25: letters (lowercase, in alphabetical order) 
  - 26: root
  - 27: hook
  - 28: empty
*/

struct Node<ToStr> {
  priv value: int,
  priv children: [Node],
  priv finite: bool
}

// implements comparable
impl Node {

  fn new(i: int) -> Node {
    let mut newNode = Node {
      value: i,
      finite: false
    };
    if i != 0 {
      newNode.children = [Node::new(0), ..28];
    }
  }

  fn get_value(&self) -> int {
    self.value
  }

  fn set_finite(&self, new_finite: bool) {
    self.finite = new_finite;
  }

  fn get_finite(&self) -> bool {
    self.finite
  }

  /**
   * Get our child nodes from our Map
   * @return Some collection of child nodes, or null if empty;
   */
  fn get_children(&self) -> [int, ..28] {
    if self.value == 0 {
      []
    } else {
      self.children
    }
  }

  /**
   * Does this node have a child with the contents c?
   * @param c the content to check
   * @return true if child list contains such child
   */
  fn has_child(&self, v: int) -> bool {
    if self.value == 0 {
      false
    } else {
      self.children[v] != 0
    }
  }

  /**
   * return the child that corresponds to the content c2
   * @param c the character to check against
   * @return a Node or null
   */
  fn get_child(&self, v: int) -> Node {
    self.children[v]
  }

  /**
   * Add a child to our Map of child nodes
   * @param c the character to add
   */
  fn add_child(&self, v: int) {
    self.children.set(v, Node::new(v));
  }

}

impl ToStr for Node {

  /**
   * A basic toString, only returns our contents
   * @return the character the node holds
   */
  fn to_str(&self) -> str {
    match(self.value) {
      0 => "~",
      1..26 => "~",
      27 => ">",
    }
  }

}

impl Eq for Node {

  fn eq(&self, other: Node) -> bool {
    self.value == other.get_value()
  }

  fn ne(&self, other: &bool) -> bool {
    self.value != other.get_value()
  }

}
