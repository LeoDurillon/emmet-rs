use super::{attribute::AttributeGroup, snippets::Snippet};

pub struct Element {
  value: String,
  childs:Vec<Element>,
  before_end: Option<String>,
  level:usize
}

impl Element {

  /**
   * Create a new element
   * Childs are an array of other element on a lower level
   * before_end is the result of a child group
   */
  pub fn new(value:&str,childs:Vec<Element>, before_end: Option<String>,level:usize) -> Self {
    Self {
      value:String::from(value),
      childs,
      before_end,
      level
    }
  }

  // Parse Element value to string
  // Recursive
  pub fn to_value(&self)->String {
    // If Element has childs 
    // Iter on all childs and parse them as string
    let mut childs = if self.childs.len() > 0 {
       format!("\n{}\n",self.childs
        .iter()
        .map(|el| 
          {
            el.to_value()
          })
        .collect::<Vec<String>>()
        .join("\n")
      )
    } else { 
      String::from("") 
    };

    // If Element has data from child group add them next to child   
    match self.before_end.clone() {
      Some(v) => {
        if v.len() > 0 {
          childs = format!("{}{}\n{}",childs,v,"\t".repeat(self.level));
        }
      }
      None => {
        if childs.len() > 0 {
          childs = format!("{}{}",childs,"\t".repeat(self.level))
        }
      }
    }
    // Get multiplier value of element
    let splitted_value=self.value.split("*").collect::<Vec<&str>>();
    
    let multiplier = match splitted_value.get(1){
      Some(v) => v.parse::<usize>().expect("Failed to parse value"),
      None => 1usize,
    };
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
    let mut tag = clean_value[0..attributes.start.unwrap_or(value.len())].to_string();
    
    if attributes.len() > 0 && tag.len() == 0 {
      tag= "div".to_string()
    }
    if tag == "Textfield" {
      println!("{:?} = {:?} = {}",childs , tag,self.before_end == None)
    }
    // Check if element is a single tag
    let result = match value.ends_with("/") && (childs.len() == 0 || self.before_end == None) && !attributes.has_text() {
      true=>{
        format!("{}<{}{}{}","\t".repeat(self.level),tag,attributes.parse(true).unwrap_or("/>".to_string()),childs)
      },
      _=>{
        format!("{}<{}{}{}</{}>","\t".repeat(self.level),tag,attributes.parse(false).unwrap_or(">".to_string()),childs,tag)
      }
    };
    vec![result;multiplier].join("\n")
  }
}
