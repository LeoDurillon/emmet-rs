use napi_derive::napi;
use statement::Statement;


pub mod statement;

#[napi]
fn expand(input:String) -> String{
  Statement::new(&input, 0).parse()
}

//Function for test purpose
pub fn start(input:&str) -> String{
  Statement::new(&input, 0).parse()
}


#[cfg(test)]
mod test {
    use crate::start;
  #[test]
  fn bug_test(){
    assert_eq!("<div>\n\t<p></p>\n</div>\n<p></p>".to_string(),start("(div>p)+p"))
  }


  #[test]
  fn should_return_html(){
    assert_eq!("<html></html>".to_string(),start("html"))
  }

  #[test]
  fn should_combine_tag(){
    assert_eq!("<html>\n\t<p></p>\n</html>".to_string(),start("html>p"))
  }

  #[test]
  fn can_be_only_siblings(){
    assert_eq!("<p></p>\n<div></div>".to_string(),start("p+div"));
    assert_eq!("<p></p>\n<div></div>\n<p></p>".to_string(),start("p+div+p"))
  }


  #[test]
  fn can_have_multiple_child(){
    assert_eq!("<html>\n\t<p></p>\n\t<a href=${1}></a>\n</html>".to_string(),start("html>p+a"))
  }

  #[test]
  fn child_can_have_child(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t</div>\n</html>".to_string();

    assert_eq!(expected,start("html>div>p+div>p"))
  }

  #[test]
  fn can_handle_tag_group(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
    assert_eq!(expected,start("html>(div>p)+div>p"));
  
    let expected = "<html>\n\t<div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
    assert_eq!(expected,start("html>(div>(div>p)+div>p)+div>p"));

    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
    assert_eq!(expected,start("html>(div>p)+div>(div>p)+p"))
  }

  #[test]
  fn element_can_multiply(){
    let expected = "<p></p>\n<p></p>\n<p></p>";
    assert_eq!(expected.to_string(),start("p*3"));

    let expected = "<div>\n\t<p></p>\n\t<p></p>\n\t<p></p>\n</div>";
    assert_eq!(expected.to_string(),start("div>p*3"))
  }

  #[test]
  fn groups_can_multiply(){
    let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>";
    assert_eq!(expected.to_string(),start("html>(div>p)*3"))
  }

  #[test]
  fn element_can_be_single() {
    let expected = "<html>\n\t<Icon/>\n</html>";
    assert_eq!(expected.to_string(),start("html>Icon/"))
  }

  #[test]
  fn element_cant_be_single_if_has_child(){
    let expected = "<html>\n\t<Icon>\n\t\t<p></p>\n\t</Icon>\n</html>";
    assert_eq!(expected.to_string(),start("html>Icon/>p"))
  }

  #[test]
  fn element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"></div>";
    assert_eq!(expected.to_string(),start("div.test.echo.bravo"))
  }
  #[test]
  fn single_element_can_have_class() {
    let expected = "<div class=\"test echo bravo\"/>";
    assert_eq!(expected.to_string(),start("div.test.echo.bravo/"))
  }


  #[test]
  fn element_can_have_props() {
    let expected = "<Table header={title} name=\"my-table\"></Table>";
    assert_eq!(expected.to_string(),start("Table:header={title}:name=my-table"))
  }

  #[test]
  fn props_can_have_dot() {
    let expected = "<Table header={table.title} name=\"my-table\"></Table>";
    assert_eq!(expected.to_string(),start("Table:header={table.title}:name=my-table"))
  }

  #[test]
  fn element_can_have_destructured_props() {
    let expected = "<Table {...props}></Table>";
    assert_eq!(expected.to_string(),start("Table:{...props}"))
  }

  #[test]
  fn single_element_can_have_props() {
    let expected = "<img src={image} alt=\"my own image\"/>";
    assert_eq!(expected.to_string(),start("img:src={image}:alt=my own image/"))
  }

  #[test]
  fn element_can_have_text() {
    let expected = "<div>My test text is awesome</div>";
    assert_eq!(expected.to_string(),start("div<My test text is awesome"))
  }

  #[test]
  fn element_can_have_all() {
    let expected = "<div class=\"test\" data={myData}>My test text is awesome</div>";
    assert_eq!(expected.to_string(),start("div.test:data={myData}<My test text is awesome"))
  }

  #[test]
  fn should_handle_complex_query(){
    let expected = "<div>\n\t<div class=\"test\">\n\t\t<p>My text</p>\n\t\t<Table header={header}></Table>\n\t</div>\n\t<footer>My footer</footer>\n</div>";

    assert_eq!(expected.to_string(),start("div>(div.test>p<My text+Table:header={header})+footer<My footer"));
    
    let expected = "<div class=\"fixed bottom-0 left-0 right-0 top-0 z-20 flex animate-appear flex-col items-center justify-center bg-black/70 backdrop-blur-md\" onClick$={onHide$}>\n\t<div class=\"flex flex-col items-center justify-center p-20\">\n\t\t<div class=\"delay animate-appear opacity-0\">\n\t\t\t<Logo class=\"size-40\" fill=\"white\"/>\n\t\t</div>\n\t\t<div>\n\t\t\t<Form class=\"mx-2 flex animate-slideB flex-col items-center gap-2 rounded-lg bg-surface p-5\" action={login}>\n\t\t\t\t<div class=\"flex items-center gap-4\">\n\t\t\t\t\t<p class=\"text-3xl font-medium\">Connexion</p>\n\t\t\t\t\t<Icon icon=\"lock\"/>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"flex flex-col gap-2 self-stretch px-3\">\n\t\t\t\t\t<Textfield label=\"Email\" type=\"email\" required name=\"email\" placeholder=\"j.dupont@example.com\" icon=\"atSign\"/>\n\t\t\t\t\t<Textfield label=\"Password\" type=\"password\" required name=\"password\" placeholder=\"Votre mot de passe\" icon=\"lock\"/>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"flex flex-col gap-2\">\n\t\t\t\t\t<Button color=\"current\" type=\"submit\">Connexion</Button>\n\t\t\t\t\t<Link href=\"#\" class=\"text-secondary transition-colors\">Mot de passe oublié ?</Link>\n\t\t\t\t</div>\n\t\t\t</Form>\n\t\t</div>\n\t</div>\n</div>";
    assert_eq!(expected.to_string(),start("div.fixed.bottom-0.left-0.right-0.top-0.z-20.flex.animate-appear.flex-col.items-center.justify-center.bg-black/70.backdrop-blur-md:onClick$={onHide$}>div.flex.flex-col.items-center.justify-center.p-20>(div.delay.animate-appear.opacity-0>Logo.size-40:fill=white/)+div>Form.mx-2.flex.animate-slideB.flex-col.items-center.gap-2.rounded-lg.bg-surface.p-5:action={login}>(div.flex.items-center.gap-4>p.text-3xl.font-medium<Connexion+Icon:icon=lock/)+(div.flex.flex-col.gap-2.self-stretch.px-3>Textfield:label=Email:type=email:required;:name=email:placeholder=j.dupont@example.com:icon=atSign/+Textfield:label=Password:type=password:required;:name=password:placeholder=Votre mot de passe:icon=lock/)+div.flex.flex-col.gap-2>Button:color=current:type=submit<Connexion+Link:href=#.text-secondary.transition-colors<Mot de passe oublié ?"))
  }

  #[test]
  fn should_handle_snippet(){
    let expected = "<a href=${1}></a>";
    assert_eq!(expected.to_string(),start("a"));

    let expected = "<>\n\t<label for=${1}></label>\n\t<input type=\"checkbox\" name=${1} id=${1}/>\n</>";
    assert_eq!(expected.to_string(),start("e>label+input:c"));
    assert_eq!(expected.to_string(),start(">label+input:c"));

    let expected = "<div class=\"foo bar\"></div>";
    assert_eq!(expected.to_string(),start(".foo.bar"));
  }
}