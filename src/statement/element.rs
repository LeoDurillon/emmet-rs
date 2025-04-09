use super::attribute::AttributeGroup;

pub struct Element {
  value: String,
  childs:Vec<Element>,
  before_end: Option<String>
}

impl Element {

  pub fn new(value:&str,childs:Vec<Element>, before_end: Option<String>) -> Self {
    Self {
      value:String::from(value),
      childs,
      before_end
    }
  }

  pub fn to_value(&self)->String {
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
      
      match self.before_end.clone() {
        Some(v) => {
          if v.len() > 0 {
            childs = format!("{}{}\n",childs,v);
          }
        }
        None => {
        }
      }

      let splitted_value=self.value.split("*").collect::<Vec<&str>>();
      let multiplier = match splitted_value.get(1){
        Some(v) => v.parse::<usize>().expect("Failed to parse value"),
        None => 1usize,
      };
      
      let value = splitted_value.get(0).unwrap().to_string();
      let attribute = AttributeGroup::new(value.replace("/", ""));
      let tag = value.replace("/", "")[0..value.len() - attribute.len() - if value.contains("/") {1} else {0}].to_string();
      let result = match value.ends_with("/") && childs.len() == 0 && !attribute.has_text() {
        true=>{
          format!("<{}{}",tag,attribute.parse(true).unwrap_or("/>".to_string()))
        },
        _=>{
          format!("<{}{}{}</{}>",tag,attribute.parse(false).unwrap_or(">".to_string()),childs,tag)
        }
      };
      vec![result;multiplier].join("\n")
  }
}
