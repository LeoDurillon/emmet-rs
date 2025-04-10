use super::attribute::AttributeGroup;

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
    
    let value = splitted_value.get(0).unwrap().to_string();
    // Create attribut group from input
    let attribute = AttributeGroup::new(value.replace("/", ""));
    // Remove attribute length from  value to get tag value
    let tag = value.replace("/", "")[0..value.len() - attribute.len() - if value.contains("/") {1} else {0}].to_string();
    
    // Check if element is a single tag
    let result = match value.ends_with("/") && childs.len() == 0 && !attribute.has_text() {
      true=>{
        format!("{}<{}{}","\t".repeat(self.level),tag,attribute.parse(true).unwrap_or("/>".to_string()))
      },
      _=>{
        format!("{}<{}{}{}</{}>","\t".repeat(self.level),tag,attribute.parse(false).unwrap_or(">".to_string()),childs,tag)
      }
    };
    vec![result;multiplier].join("\n")
  }
}
