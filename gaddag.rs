mod node;

struct GADDAG {
  separator: char,
  root: Node,
  trie: Trie
}

impl GADDAG {

  fn new() -> GADDAG {
    GADDAG {
      separator: '>',
      root: Node::new(0)
    }
  }

  fn add(&self, word: str){
    if word.length() == 0 {
      return
    };

    word = word.to_lower_case();

    let mut prefix = "";
    let mut ch = ['a'];
    let mut i = 1;
    while i < word.length() {
      prefix = word.substring(0,i);
      ch = prefix.to_char_array();
      self.reverse(ch);
      self.trie.add(ch + self.separator + word.substring(i));
      i += 1;
    }
    ch = word.to_char_array();
    self.reverse(ch);
    self.trie.add(ch + self.separator + word.substring(i));
  }

  priv fn reverse(&self, valid_data: [char]) {
    let mut i = 0;
    while i < valid_data.length/2 {
      let temp = valid_data[i];
      valid_data[i] = valid_data[valid_data.length - i - 1];
      valid_data[valid_data.length - i - 1] = temp;
      i += 1;
    }
  }

  fn find_words_with_rack_and_hook(&self, rack: [char], hook:char) -> HashSet<str> {
      let mut words = HashSet::new();
      rack.sort();

      if (hook == ' '){
        while (rack.size() > 1){
          let tile = rack.remove(0);
          self.find_words_recurse(words, "",  rack, tile, self.root, true);
        }
      } else {
        self.find_words_recurse(words, "", rack,  hook, self.root, true);
      }
      words
  }

  priv fn find_words_recurse(&self, words: HashSet<str>, word: str, rack: List<char>, hook: char, cur: Node, direction: bool) {
      let hook_node = cur.get_child(hook);

      //Base case
      //if hookNode == null {
        //return
      //};

      let hookCh = if hook == self.separator { ""; } else { hook }; //Empty character if we're the separator
      word = if direction { hookCh + word; } else { word + hookCh; }; //Direction-based concatenation

      //if we've reached the end a word, add the word to output
      if hookNode.get_finite() {
        words.add(word);
      }

      for hookNode.get_keys().each |nodeKey| {
        if (nodeKey == self.separator) {
          self.find_words_recurse(words, word, rack, self.separator, hookNode, false);
        }
        else if (rack.contains(node_key)) {
          //boolean duplicate = (rack.size() > 0 && (rack.get(nodeKey) == rack.get(rack.indexOf(nodeKey) - 1)));
          let mut new_rack = rack.clone();
          new_rack.remove(node_key);
          self.find_words_recurse(words, word, new_rack, node_key, hook_node, direction);
        }
      }
  }

}
