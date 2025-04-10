use std::fmt::Error;
use element::Element;

pub mod element;
pub mod attribute;

pub struct Statement {
  value:String,
  childs:Vec<Statement>,
  multiplier:usize,
  base_level:usize
}

impl Statement {
  /**
   * Create a new statement from input
   * Create childs based on groups
   * Childs are statement such as first_group and content after first group
   * Recursive function until end of input
   */
  pub fn new(input:&str,base_multiplier:Option<usize>,base_level:usize) -> Self{
    // Get index of closest group entry    
    let first_group_start = input.find('(');
    
    match first_group_start {
      // Group entry exist then 
      // proceed to find end of the group 
      // to create statement 
      Some(v) => {
        let mut first_group_end = 0usize;
        // Update based on number of '(' vs number of ')' found
        let mut open_pool =1;
        let mut multiplier = 1;

        // Level is based on the current level of the statement 
        // + the number of parent before the group
        let level = input[0..v].chars().filter(|x| *x == '>').collect::<Vec<char>>().len() + base_level;
        
        // Iter over chars of input after group entry
        // Update open_pool until open_pool = 0
        for (index,char) in input[v+1..].chars().enumerate() {
          match char {
            '(' => open_pool += 1,
            ')' => {
              open_pool -= 1;
              if open_pool == 0 {
                first_group_end = index+v+1;
                // If input has a '*' tries to parse number 
                // after '*' to get multiplier
                // If fails to parse ignore '*'
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

        // Create a child based on collected input from group
        // Recursive call here
        let mut childs = Vec::from([Statement::new(&input[v+1..first_group_end],Some(multiplier),level)]);
        
        // If input not finish after first group 
        // add another child containing rest of input
        // Recursive call here
        if &input[first_group_end+3..].len() > &0usize {
          childs.push(Statement::new(&input[first_group_end+2..],None,level));
        } 

        Self {
          value : input[0..v-1].to_string(),
          childs,
          multiplier:base_multiplier.unwrap_or(1),
          base_level
        }
      }
      //If no group found create a statement without child
      None => {
        Self{
          value:input.to_string(),
          childs:vec![],
          multiplier:base_multiplier.unwrap_or(1),
          base_level
        }
      }
    }
  }


  /**
   * Parse value of statement into multiple element
   * then parse the value of eache element to
   * get final snippet value
   */
  pub fn parse(&self) -> Result<String,Error> {
    // Get all levels of statement
    let mut statements = self.value.split(">").collect::<Vec<&str>>();
    // Content of last loop result
    let mut elements:Vec<Element>=Vec::from([]);

    // For all levels exept first
    // Tries to parse every siblings group as elements
    // Add content of last loop result to last tag of siblings groups
    while statements.len() > 1 {
      let level = statements.len()-1 + self.base_level;
      let last = statements.pop().expect("Failed to get statement value");
      let mut tags = parse_siblings(last);
      let last_tag = tags.pop().expect("Failed to get tag value");
      let mut element = Vec::from([Element::new(last_tag,elements,None,level )]);
      let mut result =  tags.iter()
      .map(|el| {
        Element::new(el,vec![],None,level)
      })
      .collect::<Vec<Element>>();
      
      result.append(&mut element);
      elements = Vec::from(result)
        
    }

    // Parse every childs of original statement
    // To add them at the end of first element 
    let before_end = if self.childs.len() > 0  {
      Some(self.childs.iter().fold(
        String::new(), 
        |acc,el| {format!("{}\n{}",acc,el.parse().expect("Failed to parse")) }
      ))
    } else {
      None
    };
    // Tries to get first tag value
    let mut values =parse_siblings(statements.get(0).expect("Failed to get statement value"));
    let last_tag = values.pop().expect("Failed to get tag");
    let last = Element::new(
      last_tag,
      elements,
      before_end,
      self.base_level
    ).to_value();
    let others = values.iter().map(|el| Element::new(el,vec![],None,self.base_level).to_value()).collect::<Vec<String>>().join("\n");
    let others = if others.len() > 0 { format!("{}{}",others,"\n")} else { String::new()};
    Ok(
      vec![format!("{}{}",others,last);self.multiplier].join("\n")
  )
  }
}


fn parse_siblings(statement:&str) -> Vec<&str> {
  statement.split("+").collect::<Vec<&str>>()
}