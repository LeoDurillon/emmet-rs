use super::{attribute::AttributeGroup, snippets::Snippet, Statement};

#[derive(Clone, Debug)]
pub struct Element {
  value: String,
  childs:Option<Box<Statement>>,
  level:usize
}

impl Element {

  /**
   * Create a new element
   * Childs are another statement on a lower level
   */
  pub fn new(value:String,childs:Option<Statement>,level:usize) -> Self {
    let childs = match childs {
      Some(v) => Some(Box::new(v)),
      None => None
    };
    Self {
      value:String::from(value),
      childs,
      level
    }
  }

  /**
   * Parse Element value to string 
   */ 
  pub fn to_value(&self)->String {
    // Get multiplier value of element
    let splitted_value=self.value.split("*").collect::<Vec<&str>>();
    let mut value = splitted_value.get(0).unwrap().to_string();

    let snippets = Snippet::get();
    let key = snippets.keys().filter(|key| 
      value.starts_with(*key) 
      && (
        value.split_at(key.len()).1.len() == 0 
        || value.split_at(key.len()).1.starts_with(":")
      )
    ).max();
    
    if let Some(v) = key {
      let replace = snippets.get(v).unwrap();
      value = format!("{}{}",replace,value.split_at(v.len()).1);
    }

    let clean_value = if value.ends_with("/") { value.clone()[0..value.len()-1].to_string()} else {value.clone()};
    // Create attribut group from input
    let attributes = AttributeGroup::new(&clean_value);
    // Remove attribute length from  value to get tag value
    let mut tag = clean_value[0..attributes.start.unwrap_or(clean_value.len())].to_string();
    
    if attributes.len() > 0 && tag.len() == 0 {
      tag= "div".to_string()
    };

    let child_value = match &self.childs {
      Some(v) => format!("\n{}\n{}",v.parse(),"\t".repeat(self.level)),
      None => "".to_string()
    };
    // Check if element is a single tag
    let result = match value.ends_with("/") && child_value.len() == 0 && !attributes.has_text() {
      true=>{
        format!("{}<{}{}","\t".repeat(self.level),tag,attributes.parse(true).unwrap_or("/>".to_string()))
      },
      _=>{
        format!("{}<{}{}{}</{}>","\t".repeat(self.level),tag,attributes.parse(false).unwrap_or(">".to_string()),child_value,tag)
      }
    };
    result
  }
}
