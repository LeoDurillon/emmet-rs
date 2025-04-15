use checker::input_correctly_close;
use napi_derive::napi;
use statement::Statement;


pub mod statement;
pub mod checker;

#[napi]
fn expand(input:String) -> Option<String>{
  if !input_correctly_close(&input){
    return None
  }
  Some(Statement::new(&input, 0).parse())
}


