use std::collections::HashMap;

use regex::Regex;

pub fn input_correctly_close(input:&str) -> bool {
  let mut opener:Vec<char>=Vec::new();

  let closer_opener_map = HashMap::from([
    (')','('),(']','['),('}','{'),
  ]);

  for char in input.chars() {
    match char {
      '('|'{'|'[' => {
        opener.push(char);
      }
      ')'|'}'|']' => {
        match opener.last() {
          Some(v) =>{
            if v != closer_opener_map.get(&char).unwrap_or(&'0') {
              break;
            }
            opener.pop();
          },
          None => {break;}
        }
      }
      _ => {}
    }
  }

  return opener.len() == 0;
}

pub fn input_correctly_structured(input:&str) -> bool {
  let regex:Regex = Regex::new(r"^(?:\(?[>+]?[a-zA-Z]+[.>:+]?(?:=(?:\{[a-zA-Z]+\}|[a-zA-Z]+))?\(?(?:<[\w ]+)?\)*(?:\*\d+)?)+$").unwrap();

  regex.is_match(input)
}


#[cfg(test)]
mod checker_test {
    use crate::checker::{input_correctly_close, input_correctly_structured};

  #[test]
  fn should_check_closing(){
    assert_eq!(true,input_correctly_close("html"));

    assert_eq!(true,input_correctly_close("html>p"));

    assert_eq!(true,input_correctly_close("(html>div>p)+icon"));

    assert_eq!(true,input_correctly_close("(html)+icon"));

    assert_eq!(true,input_correctly_close("(html>div>(p+div>p))+icon"));

    assert_eq!(true,input_correctly_close("(html.test.class>div:test:prop>(p+div>p))*3+icon>p"));

    assert_eq!(true,input_correctly_close("html>(div>p)*3"));

    assert_eq!(true,input_correctly_close("(div>p)*3"));

    assert_eq!(false,input_correctly_close("(div>p"));

    assert_eq!(false,input_correctly_close("div:foo={bar>p"));

    assert_eq!(false,input_correctly_close("div+("));
  }

  #[test]
  fn should_check_structure(){
    assert_eq!(false,input_correctly_structured("return function() "));

    assert_eq!(true,input_correctly_structured("html>p"));

    assert_eq!(true,input_correctly_structured("(html>div>p)+icon"));

    assert_eq!(true,input_correctly_structured("(html)+icon"));

    assert_eq!(true,input_correctly_structured("(html>div>(p+div>p))+icon"));

    assert_eq!(true,input_correctly_structured("(html.test.class>div:test:prop>(p+div>p))*3+icon>p"));

    assert_eq!(true,input_correctly_structured("html>(div>p)*3"));

    assert_eq!(true,input_correctly_structured("(div>p)*3"));

    assert_eq!(true,input_correctly_structured("(div>p"));

    assert_eq!(false,input_correctly_structured("div:foo={bar>p"));

    assert_eq!(true,input_correctly_structured("div+("));

    assert_eq!(false,input_correctly_structured("div > p"));
  }
}