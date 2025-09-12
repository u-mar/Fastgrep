#[derive(Debug)]
#[allow(dead_code)]
pub struct Arguments {
    i:bool,
    c:bool,
    l:bool,
    m:bool,
}
impl Arguments {
    pub fn build(arg:&Vec<String>) -> Arguments{
        let mut i = false;
        let mut c = false;
        let mut l = false;
        let mut m = false;
        for argu in arg {
            if argu == "-i" {
                i = true;
            }
            else if argu == "-c"{
                c = true;
            }
            else if argu == "-l"{
                l = true;
           }
           else if argu == "-m"{
            m = true;
       }
        }

        Arguments{
            i,
            c,
            l,
            m
        }

    }
}
