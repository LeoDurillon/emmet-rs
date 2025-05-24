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

  fn get_props(&self)->Vec<&str> {
    let mut result:Vec<&str> = Vec::new();
    let mut opening_dif = 0; 
    let mut last_index = 0;
    for (index,char) in self.value.chars().enumerate() {
      match char {
        '{'=>{
          opening_dif+=1;
        }
        '}'=>{
          opening_dif-=1;
        }
        ':'=>{
          if opening_dif == 0 {
            result.push(&self.value[last_index..index]);
            last_index = index+1;
          }
        }
         _=>{}
      }
    }
    result.push(&self.value[last_index..]);
    result
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
        let props = self.get_props();
        let mut result = Vec::new();

        for prop in props.iter() {
          if  props.iter().any(|el| el.split("=").collect::<Vec<&str>>().get(0).unwrap().contains(prop) && el.len() > prop.len()) {
            continue;
          }
          match prop.contains("=") {
            true => {
              let (name,value)= prop.split_at(prop.find("=").unwrap()+1);
          
              match value.contains("{") {
                true=> result.push(format!(" {}{}",name,value)),
                _=>result.push(format!(" {}\"{}\"",name,value))
              }
            } 
            _ => {
              if prop.starts_with("{") || prop.ends_with(";") {
                result.push(format!(" {}",if prop.ends_with(";") {&prop[0..prop.len()-1]} else {prop}))
              }else {
                result.push(format!(" {}=${{1}}",prop))
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
            attributes.push(Attribute {
              value:input[v+1..].to_string(), 
              attribute_type:AttributeType::Text
            });
            break;
          }
    
          attributes.push(Attribute {
            value:input[v+1..order.get(index+1).unwrap_or(&Some(input.len())).unwrap()].to_string(), 
            attribute_type:if item == &first_class {AttributeType::Class} else {AttributeType::Props}
          })
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
