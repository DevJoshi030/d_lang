use std::env;

mod ast;
mod environment;
mod evaluator;
pub mod lexer;
mod object;
mod parser;
mod repl;
mod token;

fn main() {
    println!("====================");
    println!("Welcome to D_Lang!!!");
    println!("====================");
    println!(
        r"                     
                  .[[\OOOOOO/[.
                ]/OOOOOOOOOOOOOOO\`               
           ,/OOOOOOOOOOOOOOOOOOOOOOO\`           
         /OOOOOOO/./OOOOOOOOOO`[OOOOOOO\         
       /OOOOOOO/.=OOOOOOOOOOOOOO`.\OOOOOOO       
     =OOOOOOOO` ,OOOOOOOOOOOOOOOO..=OOOOOOO\     
    /OOOOOOOO` .=OOOOOOOOOOOOOOOO^..\OOOOOOOO    
   /OOOOOOOOO. .=O/.. . ... ..[OO^..,OOOOOOOOO`  
  /OOOOOOOOOO. ..\\ .,]]]]]`...,/ ..`OOOOOOOOOO  
 ,OOOOOOOOO[`.  ..,OOOOOOOOOOOO`..   .[\OOOOOOOO 
 =OOOOOO/  . ..  . .,\OOOOO/`.... .. ....\OOOOOO`
 OOOOOO`..,/OOOO]]......]]`.. .`]/OOOO\`.,.OOOOO^
 OOOOO``/OOOO...=OO\`.,OOOO^.,OOO^...OOOOO..\OOOO
 OOOO^,OOOOOO...,OOOO\.,OO/./OOOO^. .OOOOOO^.OOOO
 \OOO^OOOOOOO\...=OOOO^....OOOOO/ .,=OOOOOOO`OOO^
 =OOOOOOOOOOOO/...=OOOO. ..OOOO`...=OOOOOOOO\OO@ 
  \OOOOOOOOOOOO\`...,OO....O/`...*/OOOOOOOOOOOO^ 
   OOOOOOOOOOOOOO\^../^....,O.,\OOOOOOOOOOOOOO/  
    OOOOOOOOOOOOOOOOO`.. ...,OOOOOOOOOOOOOOOOO   
     \OOOOOOOOOOOO/`....,`....,[OOOOOO/\OOOO/    
      ,OOOO]`.`......*/OOOOO]\,,,]`.]OOOOOO`     
        =OOOOOO@OOOOOOOOOOOOOOOOOOOOOOOOO`       
           \OOOOOOOOOOOOOOOOOO@OOO@OOO/          
             .\OOOOOOOOOOOO@OOOOOO/`             
                  .[[\OOOOOO/[.                  
        "
    );

    let filename = match env::args().collect::<Vec<String>>().get(1) {
        Some(f) => f.clone(),
        None => "".to_string(),
    };

    if filename == "" {
        repl::run();
    } else {
        repl::run_file(filename);
    }
}
