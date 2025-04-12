#[derive(PartialEq, Eq)]
enum AttributeType {
  Class,
  Props,
  Text
}

pub struct Attribute {
  value:String,
  attribute_type:AttributeType
}

impl Attribute {
  pub fn len(&self) -> usize {
    self.value.len()
  }

  /**
   * Parse attribute to string based on attribute type
   */
  pub fn parse(&self) -> String{
    match self.attribute_type {
      AttributeType::Text => format!(">{}",self.value),
      AttributeType::Class => {
        let result = self.value.split(".").collect::<Vec<&str>>();
        format!(" class=\"{}\"",result.join(" "))
      },
      AttributeType::Props => {
        let splitted = self.value.split(":").collect::<Vec<&str>>();
        let mut result = Vec::new();

        for (index ,props) in splitted.iter().enumerate() {
          if let Some(_) = splitted[index+1..].iter().find(|x| x.starts_with(&props.split_at(props.find("=").unwrap_or(props.len()-1)+1).0)) {
            continue;
          }
          match props.contains("=") {
            true => {
              let (prop,value)= props.split_at(props.find("=").unwrap()+1);
          
              match value.contains("{") {
                true=> result.push(format!(" {}{}",prop,value)),
                _=>result.push(format!(" {}\"{}\"",prop,value))
              }
            } 
            _ => {
              if props.starts_with("{") || props.ends_with(";") {
                result.push(format!(" {}",if props.ends_with(";") {&props[0..props.len()-1]} else {props}))
              }else {
                result.push(format!(" {}=${{1}}",props))
              }
            }
          }
          
        }
        result.join("")
      }
    }
  }
}


pub struct AttributeGroup {
  pub start:Option<usize>,
  attributes: Vec<Attribute>
}

impl AttributeGroup {
  /**
   * Create a new attribute group from input
   */
  pub fn new(input:&String) -> Self {
    // Search for first class definition
    // Such as a '.' that is not in a props definition 
    let first_class =match input.chars().enumerate().find(|(i,c)| 
      c==&'.' 
      && !(input[0..*i].contains(":") && (input[i+1..].contains("}") || input[i+1..].contains(":"))) // Check if item is variable call
      && (if let Some(v) = i.checked_sub(1) {&input[v..*i] != "."} else {true}) && &input[i+1..i+2] != "." // Check for destructuration
    ) {
      Some(v)=>Some(v.0),
      None=>None,
    };

    let first_prop = input.find(":");
    let first_text = input.find("<");

    let mut attributes:Vec<Attribute> = Vec::new();
    // Get element in order of appearance
    let mut order = Vec::from([first_class,first_prop,first_text]);
    order.sort();

    // For each item 
    // Create attribute from index to next item index
    // if item index = 0 attribute is not referenced in input
    // Stop iteration after text as text could be anything
    for (index,item) in order.iter().enumerate() {
      match item {
        None => continue,
        Some(v) => {
          if item == &first_text {
            attributes.push(Attribute {value:input[v+1..].to_string(), attribute_type:AttributeType::Text});
            break;
          }
    
          attributes.push(Attribute {value:input[v+1..order.get(index+1).unwrap_or(&Some(input.len())).unwrap()].to_string(), attribute_type:if item == &first_class {AttributeType::Class} else {AttributeType::Props}})
        }
      }

     
    }

    Self {
      start:order.iter().find(|index| if let Some(_) = index {true} else {false}).unwrap_or(&None).clone(),
      attributes
    }
  }

  /**
   * Get total len of all attribute 
  */ 
  pub fn len(&self) -> usize {
    if self.attributes.len() == 0 {
      return 0
    }
    self.attributes.iter().fold(self.attributes.len(),|acc,el| acc+el.len() )
  }

  /**
   * Check if text attribute exist in group
   */
  pub fn has_text(&self) -> bool {
    match self.attributes.iter().find(|el| el.attribute_type == AttributeType::Text) {
      Some(_) => true,
      None => false
    }
  }

  /**
   * Parse all attribute and close tag based on tag type
   */
  pub fn parse(&self,is_single:bool) -> Option<String>{
    if self.attributes.len() == 0 {
      return None
    }
    let mut result = self.attributes.iter().fold(String::new(),|acc,el| format!("{}{}",acc,el.parse()));
    
    if !result.contains(">") {
      result = format!("{}{}",result,if is_single {"/>"} else {">"})
    }
    Some(result)
  }
}
