use std::ffi::{c_char, CStr, CString};

use statement::Statement;


pub mod statement;

#[unsafe(no_mangle)]
pub extern "C" fn expand(input:*const c_char) -> *const c_char {
  let input = unsafe {match CStr::from_ptr(input).to_str() {
    Ok(v)=>v,
    Err(_) => panic!("Failed to parse value")
  }};
  let res = Statement::new(input, None).parse();
  let string = CString::from_vec_with_nul(res.bytes().collect::<Vec<u8>>()).expect("Failed to parse String to CString");
  string.as_ptr()
}

//Function for test purpose
pub fn start(input:&str) -> String{
  Statement::new(&input, None).parse()
}


#[cfg(test)]
mod test {
    use crate::start;

  #[test]
  fn should_return_html(){
    assert_eq!("<html></html>".to_string(),start("html"))
  }

  #[test]
  fn should_combine_tag(){
    assert_eq!("<html>\n<p></p>\n</html>".to_string(),start("html>p"))
  }

  #[test]
  fn can_have_multiple_child(){
    assert_eq!("<html>\n<p></p>\n<a></a>\n</html>".to_string(),start("html>p+a"))
  }

  #[test]
  fn child_can_have_child(){
    let expected = "<html>\n<div>\n<p></p>\n<div>\n<p></p>\n</div>\n</div>\n</html>".to_string();

    assert_eq!(expected,start("html>div>p+div>p"))
  }

  #[test]
  fn can_handle_tag_group(){
    let expected = "<html>\n<div>\n<p></p>\n</div>\n<div>\n<p></p>\n</div>\n</html>".to_string();
    assert_eq!(expected,start("html>(div>p)+div>p"))
  }

  #[test]
  fn element_can_multiply(){
    let expected = "<p></p>\n<p></p>\n<p></p>";
    assert_eq!(expected,start("p*3"))
  }

  #[test]
  fn groups_can_multiply(){
    let expected = "<html>\n<div>\n<p></p>\n</div>\n<div>\n<p></p>\n</div>\n<div>\n<p></p>\n</div>\n</html>";
    assert_eq!(expected,start("html>(div>p)*3"))
  }

  #[test]
  fn element_can_be_single() {
    let expected = "<html>\n<Icon/>\n</html>";
    assert_eq!(expected,start("html>Icon/"))
  }

  #[test]
  fn element_cant_be_single_if_has_child(){
    let expected = "<html>\n<Icon>\n<p></p>\n</Icon>\n</html>";
    assert_eq!(expected,start("html>Icon/>p"))
  }

  #[test]
  fn element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"></div>";
    assert_eq!(expected,start("div.test.echo.bravo"))
  }
  #[test]
  fn single_element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"/>";
    assert_eq!(expected,start("div.test.echo.bravo/"))
  }


  #[test]
  fn element_can_have_props() {
    let expected = "<Table header={title} name=\"my-table\"></Table>";
    assert_eq!(expected,start("Table:header={title}:name=my-table"))
  }

  #[test]
  fn element_can_have_destructured_props() {
    let expected = "<Table {...props}></Table>";
    assert_eq!(expected,start("Table:{...props}"))
  }

  #[test]
  fn single_element_can_have_props() {
    let expected = "<img src={image} alt=\"my own image\"/>";
    assert_eq!(expected,start("img:src={image}:alt=my own image/"))
  }

  #[test]
  fn element_can_have_text() {
    let expected = "<div>My test text is awesome</div>";
    assert_eq!(expected,start("div<My test text is awesome"))
  }

  #[test]
  fn element_can_have_all() {
    let expected = "<div class=\"test\" data={myData}>My test text is awesome</div>";
    assert_eq!(expected,start("div.test:data={myData}<My test text is awesome"))
  }
}