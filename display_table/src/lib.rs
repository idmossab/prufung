use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}


impl Table {
    pub fn new() -> Table {
           Table {
                  headers: Vec::new(),
                  body: Vec::new()
             }  
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec())
    }
}



impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.headers.is_empty(){
            return Ok(())  ;
        }

        let mut withs : Vec<_> = self.headers.iter().map(|row| row.len() ).collect();

         
        for row in self.body.iter(){
           for (i,cell) in row.iter().enumerate(){
            withs[i] = withs[i].max(cell.len())
        }  
        }
    // write!(f,"*{:?}",withs)?; 
       write!(f,"|")?;


       for (i,row) in self.headers.iter().enumerate(){
            let lspace = (withs[i] + 2 - row.len() ) / 2 ;
        let rspace = (withs[i] + 2 - row.len() ) - lspace ;
          write!(f,"{}{}{}|"," ".repeat(lspace as usize),row," ".repeat(rspace as usize),)?;   
       }
        writeln!(f,"")?;

        write!(f,"|")?;
       for (i,cell) in withs.iter().enumerate(){
         write!(f,"{}","-".repeat(*cell+2 as usize))?;
         if i < withs.len()-1 {
            write!(f,"+")?;
         }else{
            write!(f,"|")?;
         }
       }
        writeln!(f,"")?;  
        
       for row in self.body.iter(){
         write!(f,"|")?;
         for (i,cell) in row.iter().enumerate(){
                let lspace = (withs[i] + 2 - cell.len() ) / 2 ;
             let rspace = (withs[i] + 2 - cell.len() ) - lspace ;
          write!(f,"{}{}{}|"," ".repeat(lspace as usize),cell," ".repeat(rspace as usize),)?;   
         }
            writeln!(f,"")?;
       }
        Ok(())   
    }
}