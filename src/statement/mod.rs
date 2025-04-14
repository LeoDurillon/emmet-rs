use element::Element;

pub mod element;
pub mod attribute;
pub mod snippets;
#[derive(Clone, Debug)]
pub struct Statement {
  value:Element,
  sibling:Option<Box<Statement>>,
  multiplier:usize,
}

impl Statement {
  pub fn new(input:&str,base_level:usize) -> Self {
    let ((value,child,multiplier),sibling) = Self::parse_input(input);

    let element = Element::new(
      value.to_string(), 
      if child.len() > 0 {Some(Statement::new(child,base_level+1))} 
      else {None}, 
      base_level
    );

    let siblings = if sibling.len() > 0 {Some(Box::new(Statement::new(sibling,base_level)))} else {None};

    Self {
      value:element,
      sibling:siblings,
      multiplier
    }
  }

  /**
   * Parse input to get :
   * element of actual statement
   * childs of element
   * siblings of element
   * multiplier of element
   * return ((element,childs,multiplier),siblings)
   */
  fn parse_input(input:&str) -> ((&str,&str,usize),&str) {

    let first_down = input.split_at(input.find(">").unwrap_or(input.len()));
    let first_sibling = input.split_at(input.find("+").unwrap_or(input.len()));
    let first_opening =input.split_at(input.find("(").unwrap_or(input.len()));
    let mut multiplier = 1;
  
    let mut order = [first_down.0.len(),first_sibling.0.len(),first_opening.0.len()];
    order.sort();
    
    let first = order.get(0).unwrap();
    if first == &first_down.0.len() {
      let element = first_down.0;
      if element.contains("*") {
        multiplier = element.split_at(element.find("*").unwrap_or(0)+1).1.parse::<usize>().unwrap_or(1);
      }
      if first_down.0.len() < input.len() {
        return ((element.split_at(element.find("*").unwrap_or(element.len())).0,&input[first_down.0.len()+1..],multiplier),"")
      } else {
        return ((element.split_at(element.find("*").unwrap_or(element.len())).0,"",multiplier),"")
      }
    }
  
    if first == &first_sibling.0.len() {
      let element = first_sibling.0;
      if element.contains("*") {
        multiplier = element.split_at(element.find("*").unwrap_or(0)+1).1.parse::<usize>().unwrap_or(1);
      }
      return ((element.split_at(element.find("*").unwrap_or(element.len())).0,"",multiplier), &first_sibling.1[1..])
    }
  
    if first == &first_opening.0.len() {
      let closing = Self::find_closing_index(&input[first_opening.0.len()+1..]);
      let first_down = input[first_opening.0.len()..].split_at(input.find(">").unwrap_or(input.len()));
      let first_sibling = input[first_opening.0.len()..].split_at(input.find("+").unwrap_or(input.len()));
      let mut inner_order = [first_down.0.len(),first_sibling.0.len()];
      inner_order.sort();
      if input[closing..].find("*").unwrap_or(input.len()) < input[closing..].find("+").unwrap_or(input.len())  {
        multiplier = input[closing..][input[closing..].find('*').unwrap_or(0)+1..input[closing..].find('+').unwrap_or(input[closing..].len())].to_string().parse::<usize>().unwrap_or(1)
        
      }
  
      let inner_first = inner_order.get(0).unwrap();
      if inner_first == &first_down.0.len() {
        return ((&first_down.0[first_opening.0.len()+1..],&input[first_opening.0.len() + first_down.0.len()+1..closing+1],multiplier), &input[closing..].split_at(input[closing..].find("+").unwrap_or(input[closing+1..].len())+1).1)
      }
    
      if inner_first == &first_sibling.0.len() {
        return ((&first_sibling.0[first_opening.0.len()+1..closing+1],"",multiplier), &first_sibling.1[1..])
      }
    }
  
    return ((input,"",multiplier),"")
  }

  /**
   * Get index of current parentheses closing
   */
  fn find_closing_index(input:&str) -> usize{
    let mut opening_dif=1;
    let mut result = input.len();
    for (index,char) in input.chars().enumerate() {
      match char {
        '(' =>{
          opening_dif+=1;
        }
        ')' =>{
          if opening_dif == 1 {
            result = index;
            break;
          }
          opening_dif-=1;
        }
        _=>{}
      }
    }
  
    result
  }

  /**
   * Parse statement into final snippet value
   */
  pub fn parse(&self) -> String {
    let mut result = self.value.to_value();
    for _ in 1..self.multiplier {
      result = format!("{}\n{}",result,self.value.to_value());
    }

    match &self.sibling {
      Some(v)=>{
        return format!("{}\n{}",result,v.parse())
      }
      None => {
        return result
      }
    }
  }
}






#[cfg(test)]
mod test {
    use crate::statement::Statement;

  #[test]
  fn test_parser() {
    let expected = (("html","",1),"");
    assert_eq!(expected,Statement::parse_input("html"));

    let expected = (("html","",3),"");
    assert_eq!(expected,Statement::parse_input("html*3"));

    let expected = (("html","p",1),"");
    assert_eq!(expected,Statement::parse_input("html>p"));

    let expected = (("html","",1),"p");
    assert_eq!(expected,Statement::parse_input("html+p"));

    let expected = (("html","",1),"div>p+icon");
    assert_eq!(expected,Statement::parse_input("html+div>p+icon"));

    let expected = (("html","div>p+icon",1),"");
    assert_eq!(expected,Statement::parse_input("html>div>p+icon"));

    let expected = (("html","div>p",1),"icon");
    assert_eq!(expected,Statement::parse_input("(html>div>p)+icon"));

    let expected = (("html","",1),"icon");
    assert_eq!(expected,Statement::parse_input("(html)+icon"));

    let expected = (("html","div>(p+div>p)",1),"icon");
    assert_eq!(expected,Statement::parse_input("(html>div>(p+div>p))+icon"));

    let expected = (("html.test.class","div:test:prop>(p+div>p)",3),"icon>p");
    assert_eq!(expected,Statement::parse_input("(html.test.class>div:test:prop>(p+div>p))*3+icon>p"));
    
    let expected = (("html","(div>p)*3",1),"");
    assert_eq!(expected,Statement::parse_input("html>(div>p)*3"));
  
    let expected = (("div","p",3),"");
    assert_eq!(expected,Statement::parse_input("(div>p)*3"));
  }
}