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
  
    let mut order = [first_down.0.len(),first_sibling.0.len(),if first_opening.0.len() == 0 {first_opening.0.len()} else {usize::MAX}];
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
mod test_statement {
  #[cfg(test)]
  mod parser {
      use crate::statement::Statement;
  
    #[test]
    fn should_correctly_parse_output() {
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

  #[cfg(test)]
  mod output {
    use crate::statement::Statement;
    
    #[test]
    fn should_return_html(){
      assert_eq!("<html></html>".to_string(),Statement::new("html",0).parse())
    }
    
    #[test]
    fn should_combine_tag(){
      assert_eq!("<html>\n\t<p></p>\n</html>".to_string(),Statement::new("html>p",0).parse());
      assert_eq!("<html>\n\t<p></p>\n\t<a href=${1}></a>\n</html>".to_string(),Statement::new("html>p+a",0).parse());
      
      let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t</div>\n</html>".to_string();
      assert_eq!(expected,Statement::new("html>div>p+div>p",0).parse());
      
      assert_eq!("<div>\n\t<p></p>\n</div>\n<p></p>".to_string(),Statement::new("(div>p)+p", 0).parse())
    }
  
    #[test]
    fn can_be_only_siblings(){
      assert_eq!("<p></p>\n<div></div>".to_string(),Statement::new("p+div",0).parse());
      assert_eq!("<p></p>\n<div></div>\n<p></p>".to_string(),Statement::new("p+div+p",0).parse())
    }
  
    #[test]
    fn can_handle_tag_group(){
      let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
      assert_eq!(expected,Statement::new("html>(div>p)+div>p",0).parse());
    
      let expected = "<html>\n\t<div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
      assert_eq!(expected,Statement::new("html>(div>(div>p)+div>p)+div>p",0).parse());
  
      let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t\t<p></p>\n\t</div>\n</html>".to_string();
      assert_eq!(expected,Statement::new("html>(div>p)+div>(div>p)+p",0).parse())
    }
  
    #[test]
    fn element_can_multiply(){
      let expected = "<p></p>\n<p></p>\n<p></p>";
      assert_eq!(expected.to_string(),Statement::new("p*3",0).parse());
  
      let expected = "<div>\n\t<p></p>\n\t<p></p>\n\t<p></p>\n</div>";
      assert_eq!(expected.to_string(),Statement::new("div>p*3",0).parse());
      
      let expected = "<html>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n\t<div>\n\t\t<p></p>\n\t</div>\n</html>";
      assert_eq!(expected.to_string(),Statement::new("html>(div>p)*3",0).parse())
    }
  
    #[test]
    fn single_element() {
      // Can be single
      let expected = "<html>\n\t<Icon/>\n</html>";
      assert_eq!(expected.to_string(),Statement::new("html>Icon/",0).parse());
      // Can't be Single if has child
      let expected = "<html>\n\t<Icon>\n\t\t<p></p>\n\t</Icon>\n</html>";
      assert_eq!(expected.to_string(),Statement::new("html>Icon/>p",0).parse())
    }

    #[test]
    fn element_can_have_props() {
      // Classes
      let expected = "<div class=\"test echo bravo\"></div>";
      assert_eq!(expected.to_string(),Statement::new("div.test.echo.bravo",0).parse());
      // Single element Classes
      let expected = "<div class=\"test echo bravo\"/>";
      assert_eq!(expected.to_string(),Statement::new("div.test.echo.bravo/",0).parse());
      // Props
      let expected = "<Table header={title} name=\"my-table\"></Table>";
      assert_eq!(expected.to_string(),Statement::new("Table:header={title}:name=my-table",0).parse());
      // Single Element props
      let expected = "<img src={image} alt=\"my own image\"/>";
      assert_eq!(expected.to_string(),Statement::new("img:src={image}:alt=my own image/",0).parse());
      // Props with dot
      let expected = "<Table header={table.title} name=\"my-table\"></Table>";
      assert_eq!(expected.to_string(),Statement::new("Table:header={table.title}:name=my-table",0).parse());
      // Props is function
      let expected = "<Table table={table.get()}></Table>";
      assert_eq!(expected.to_string(),Statement::new("Table:table={table.get()}",0).parse());
      // Destructured
      let expected = "<Table {...props}></Table>";
      assert_eq!(expected.to_string(),Statement::new("Table:{...props}",0).parse());
      // Text
      let expected = "<div>My test text is awesome</div>";
      assert_eq!(expected.to_string(),Statement::new("div<My test text is awesome",0).parse());
      // All
      let expected = "<div class=\"test\" data={myData}>My test text is awesome</div>";
      assert_eq!(expected.to_string(),Statement::new("div.test:data={myData}<My test text is awesome",0).parse());
    }
  
    #[test]
    fn should_handle_complex_query(){
      let expected = "<div>\n\t<div>\n\t\t<div>\n\t\t\t<p></p>\n\t\t</div>\n\t\t<p></p>\n\t</div>\n\t<p></p>\n</div>";
  
      assert_eq!(expected.to_string(),Statement::new("div>(div>(div>p)+p)+p",0).parse());
      
      let expected = "<div class=\"fixed bottom-0 left-0 right-0 top-0 z-20 flex animate-appear flex-col items-center justify-center bg-black/70 backdrop-blur-md\" onClick$={onHide$}>\n\t<div class=\"flex flex-col items-center justify-center p-20\">\n\t\t<div class=\"delay animate-appear opacity-0\">\n\t\t\t<Logo class=\"size-40\" fill=\"white\"/>\n\t\t</div>\n\t\t<div>\n\t\t\t<Form class=\"mx-2 flex animate-slideB flex-col items-center gap-2 rounded-lg bg-surface p-5\" action={login}>\n\t\t\t\t<div class=\"flex items-center gap-4\">\n\t\t\t\t\t<p class=\"text-3xl font-medium\">Connexion</p>\n\t\t\t\t\t<Icon icon=\"lock\"/>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"flex flex-col gap-2 self-stretch px-3\">\n\t\t\t\t\t<Textfield label=\"Email\" type=\"email\" required name=\"email\" placeholder=\"j.dupont@example.com\" icon=\"atSign\"/>\n\t\t\t\t\t<Textfield label=\"Password\" type=\"password\" required name=\"password\" placeholder=\"Votre mot de passe\" icon=\"lock\"/>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"flex flex-col gap-2\">\n\t\t\t\t\t<Button color=\"current\" type=\"submit\">Connexion</Button>\n\t\t\t\t\t<Link href=\"#\" class=\"text-secondary transition-colors\">Mot de passe oublié ?</Link>\n\t\t\t\t</div>\n\t\t\t</Form>\n\t\t</div>\n\t</div>\n</div>";
      assert_eq!(expected.to_string(),Statement::new("div.fixed.bottom-0.left-0.right-0.top-0.z-20.flex.animate-appear.flex-col.items-center.justify-center.bg-black/70.backdrop-blur-md:onClick$={onHide$}>div.flex.flex-col.items-center.justify-center.p-20>(div.delay.animate-appear.opacity-0>Logo.size-40:fill=white/)+div>Form.mx-2.flex.animate-slideB.flex-col.items-center.gap-2.rounded-lg.bg-surface.p-5:action={login}>(div.flex.items-center.gap-4>p.text-3xl.font-medium<Connexion+Icon:icon=lock/)+(div.flex.flex-col.gap-2.self-stretch.px-3>Textfield:label=Email:type=email:required;:name=email:placeholder=j.dupont@example.com:icon=atSign/+Textfield:label=Password:type=password:required;:name=password:placeholder=Votre mot de passe:icon=lock/)+div.flex.flex-col.gap-2>Button:color=current:type=submit<Connexion+Link:href=#.text-secondary.transition-colors<Mot de passe oublié ?",0).parse())
    }
  
    #[test]
    fn should_handle_snippet(){
      let expected = "<a href=${1}></a>";
      assert_eq!(expected.to_string(),Statement::new("a",0).parse());
  
      let expected = "<>\n\t<label for=${1}></label>\n\t<input type=\"checkbox\" name=${1} id=${1}/>\n</>";
      assert_eq!(expected.to_string(),Statement::new("e>label+input:c",0).parse());
      assert_eq!(expected.to_string(),Statement::new(">label+input:c",0).parse());
  
      let expected = "<div class=\"foo bar\"></div>";
      assert_eq!(expected.to_string(),Statement::new(".foo.bar",0).parse());
    }
  }
}


