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
        for props in splitted {
          match props.contains("=") {
            true => {
              let (prop,value)= props.split_at(props.find("=").unwrap()+1);
          
              match value.contains("{") {
                true=> result.push(format!(" {}{}",prop,value)),
                _=>result.push(format!(" {}\"{}\"",prop,value))
              }
            } 
            _ => {
              result.push(format!(" {}",props))
            }
          }
          
        }
        result.join("")
      }
    }
  }
}


pub struct AttributeGroup {
  attributes: Vec<Attribute>
}

impl AttributeGroup {
  pub fn new(input:String) -> Self {
    let first_class = input.chars().enumerate().find(|(i,c)| c==&'.' && &input[i-1..*i] != "." && &input[i+1..*i+2] != "." ).unwrap_or((0,'a')).0;
    let first_prop = input.find(":").unwrap_or(0);
    let first_text = input.find("<").unwrap_or(0);

    let mut attributes:Vec<Attribute> = Vec::new();
    let mut order = Vec::from([first_class,first_prop,first_text]);
    order.sort();

    for (index,item) in order.iter().enumerate() {
      if item == &0 {
        continue;
      }

      if item == &first_text {
        attributes.push(Attribute {value:input[item+1..].to_string(), attribute_type:AttributeType::Text});
        break;
      }

      attributes.push(Attribute {value:input[item+1..*order.get(index+1).unwrap_or(&input.len())].to_string(), attribute_type:if item == &first_class {AttributeType::Class} else {AttributeType::Props}})
    }

    Self {
      attributes
    }
  }

  pub fn len(&self) -> usize {
    if self.attributes.len() == 0 {
      return 0
    }
    self.attributes.iter().fold(self.attributes.len(),|acc,el| acc+el.len() )
  }

  pub fn has_text(&self) -> bool {
    match self.attributes.iter().find(|el| el.attribute_type == AttributeType::Text) {
      Some(_) => true,
      None => false
    }
  }

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
