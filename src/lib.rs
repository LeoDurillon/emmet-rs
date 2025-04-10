use napi_derive::napi;
use statement::Statement;


pub mod statement;

#[napi]
fn expand(input:String) -> Option<String>{
  match Statement::new(&input, None,0).parse() {
    Ok(v) => Some(v),
    Err(_) => None
  }
}

//Function for test purpose
pub fn start(input:&str) -> Option<String>{
  match Statement::new(&input, None,0).parse() {
    Ok(v) => Some(v),
    Err(_) => None
  }
}


#[cfg(test)]
mod test {
    use crate::start;

  #[test]
  fn should_return_html(){
    assert_eq!(Some("<html></html>".to_string()),start("html"))
  }

  #[test]
  fn should_combine_tag(){
    assert_eq!(Some("<html>\n\t<p></p>\n</html>".to_string()),start("html>p"))
  }

  #[test]
  fn can_be_only_siblings(){
    assert_eq!(Some("<p></p>\n<div></div>".to_string()),start("p+div"))
  }


  #[test]
  fn can_have_multiple_child(){
    assert_eq!(Some("<html>\n\t<p></p>\n\t<a></a>\n</html>".to_string()),start("html>p+a"))
  }

  #[test]
  fn child_can_have_child(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t</div>\n</html>".to_string();

    assert_eq!(Some(expected),start("html>div>p+div>p"))
  }

  #[test]
  fn can_handle_tag_group(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
    assert_eq!(Some(expected),start("html>(div>p)+div>p"))
  }

  #[test]
  fn element_can_multiply(){
    let expected = "<p></p>\n<p></p>\n<p></p>";
    assert_eq!(Some(expected.to_string()),start("p*3"))
  }

  #[test]
  fn groups_can_multiply(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>";
    assert_eq!(Some(expected.to_string()),start("html>(div>p)*3"))
  }

  #[test]
  fn element_can_be_single() {
    let expected = "<html>\n\t<Icon/>\n</html>";
    assert_eq!(Some(expected.to_string()),start("html>Icon/"))
  }

  #[test]
  fn element_cant_be_single_if_has_child(){
    let expected = "<html>\n\t<Icon>\n\t\t<p></p>\n\t</Icon>\n</html>";
    assert_eq!(Some(expected.to_string()),start("html>Icon/>p"))
  }

  #[test]
  fn element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"></div>";
    assert_eq!(Some(expected.to_string()),start("div.test.echo.bravo"))
  }
  #[test]
  fn single_element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"/>";
    assert_eq!(Some(expected.to_string()),start("div.test.echo.bravo/"))
  }


  #[test]
  fn element_can_have_props() {
    let expected = "<Table header={title} name=\"my-table\"></Table>";
    assert_eq!(Some(expected.to_string()),start("Table:header={title}:name=my-table"))
  }

  #[test]
  fn element_can_have_destructured_props() {
    let expected = "<Table {...props}></Table>";
    assert_eq!(Some(expected.to_string()),start("Table:{...props}"))
  }

  #[test]
  fn single_element_can_have_props() {
    let expected = "<img src={image} alt=\"my own image\"/>";
    assert_eq!(Some(expected.to_string()),start("img:src={image}:alt=my own image/"))
  }

  #[test]
  fn element_can_have_text() {
    let expected = "<div>My test text is awesome</div>";
    assert_eq!(Some(expected.to_string()),start("div<My test text is awesome"))
  }

  #[test]
  fn element_can_have_all() {
    let expected = "<div class=\"test\" data={myData}>My test text is awesome</div>";
    assert_eq!(Some(expected.to_string()),start("div.test:data={myData}<My test text is awesome"))
  }

  #[test]
  fn should_handle_complex_query(){
    let expected = "<div>\n\t<div class=\"test\">\n\t\t<p>My text</p>\n\t\t<Table header={header}></Table>\n\t</div>\n\t<footer>My footer</footer>\n</div>";

    assert_eq!(Some(expected.to_string()),start("div>(div.test>p<My text+Table:header={header})+footer<My footer"))
  }
}