use element::Element;

pub mod element;
pub mod attribute;

pub struct Statement {
  value:String,
  childs:Vec<Statement>,
  multiplier:usize
}

impl Statement {
  pub fn new(input:&str,base_multiplier:Option<usize>) -> Self{
    let first_group_start = input.find('(');
    
    match first_group_start {
      Some(v) => {
        let mut first_group_end = 0usize;
        let mut open_pool =1;
        let mut multiplier = 1;
        for (index,char) in input[v+1..].chars().enumerate() {
          match char {
            '(' => open_pool += 1,
            ')' => {
              open_pool -= 1;
              println!("{}",open_pool);
              if open_pool == 0 {
                first_group_end = index+v+1;
                if input[index+v+2..].contains("*") {
                  multiplier = input[index+v+3..index+v+4].parse::<usize>().unwrap_or(1);
                }
                break;
              }
              continue;
            }
            _ => continue
          }
        }

        let mut childs = Vec::from([Statement::new(&input[v+1..first_group_end],Some(multiplier))]);
        if &input[first_group_end+3..].len() > &0usize {
          childs.push(Statement::new(&input[first_group_end+2..],None));
        } 

        Self {
          value : input[0..v-1].to_string(),
          childs,
          multiplier:base_multiplier.unwrap_or(1),
        }
      }
      None => {
        Self{
          value:input.to_string(),
          childs:vec![],
          multiplier:base_multiplier.unwrap_or(1)
        }
      }
    }
  }



  pub fn parse(&self) -> String {
    let mut statement = self.value.split(">").collect::<Vec<&str>>();
    let mut elements:Vec<Element>=Vec::from([]);  
    while statement.len() > 1 {
      let last = statement.pop().unwrap();
      let mut tags = parse_siblings(last);
      let last_tag = tags.pop().unwrap();
      let mut element = Vec::from([Element::new(last_tag,elements,None )]);
      let mut result =  tags.iter()
      .map(|el| {
        Element::new(el,vec![],None)
      })
      .collect::<Vec<Element>>();
      
      result.append(&mut element);
      elements = Vec::from(result)
        
    }
    vec![Element::new(
      statement.get(0).unwrap(),
      elements,
      Some(self.childs.iter().fold(
        String::new(), 
        |acc,el| {format!("{}\n{}",acc,el.parse()) }
      ))
    ).to_value();self.multiplier].join("\n")
  }
}


fn parse_siblings(statement:&str) -> Vec<&str> {
  statement.split("+").collect::<Vec<&str>>()
}