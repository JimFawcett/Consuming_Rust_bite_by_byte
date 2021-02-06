#[derive(Debug)]
struct Error;

#[derive(Debug)]
struct Demo;
impl Demo {
    fn do_int(&self, i:i32) -> &Self {
        print!("\n  my argument is {}", i);
        &self
    }
    fn do_float(&self, f:f64) -> &Self {
        print!("\n  my argument is {}", f);
        &self
    }
    fn do_vec(&self, v:Vec<i32>) -> &Self {
        print!("\n  my argument is {:?}", v);
        &self
    }
    fn do_err(&self, p:bool) -> Result<String, Error> {
        let e = Error {};
        if p { Ok("no error".to_string()) } else { Err(e) }
    } 
}

fn main() -> Result<(),Error> {
    let d = Demo {};
    let rslt = d.do_int(42)
                .do_float(3.14159)
                .do_vec(vec![1,2,3])
                .do_err(true);
    print!("\n  rslt = {:?}", rslt);
    
    /*-----------------------------------------------
      Bubbling up Errors
        If do_err returns Ok(value) then bind 
        value to rslt,
        else return Err(Error).
    */
    let rslt = d.do_int(42).do_float(3.1415927).do_err(true)?;
    print!("\n  rslt = {:?}", rslt);
    let rslt = d.do_int(42).do_float(3.1415927).do_err(false)?;
    print!("\n  rslt = {:?}", rslt);
    Ok(())
}