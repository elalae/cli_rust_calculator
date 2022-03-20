use std::env;

#[cfg(test)]
mod test{
    use crate::add;
    use crate::avg;
    use crate::mul;
    use crate::sort;
    use crate::sub;

    #[test]
    fn test_add(){
        let mut v1= vec![];
        v1.push(6.0);
        v1.push(3.0);
        v1.push(1.0);
       assert_eq! (add(v1),10.0);
    }

    #[test]
    fn test_sub(){
        let mut v1= vec![];
        v1.push(6.0);
        v1.push(3.0);
        v1.push(1.0);
       assert_eq! (sub(v1),2.0);
    }

    #[test]
    fn test_mul(){
        let v1=vec![2.0,2.0,2.0];
       assert_eq! (mul(v1),8.0);
    }

     
    #[test]
    fn test_sort(){
        let mut v1=vec![4.0,2.0,3.0];
       assert_eq! (sort(&mut v1),vec![2.0,3.0,4.0]);
    }

    #[test]
    fn test_avg(){
        let v1=vec![2.0,2.0,2.0];
       assert_eq! (avg(v1),2.0);
    }
        
}


fn add(v1: Vec<f64>)-> f64{
    let sum:f64= v1.iter().sum();
    return sum;
    
}

fn sub(v1: Vec<f64>)-> f64{

    let mut diff:f64=v1[0];
    
    for i in 1..=v1.len(){
        diff=diff-v1[i];
    }

    return diff;
    
}

fn mul(v1: Vec<f64>)-> f64{
    let mut inm:f64=1.0;
    for i in v1.iter(){
        inm=inm*i;
    }
    
    return inm;
}





fn div(v1: Vec<f64>) -> Option<f64> {
let mut sdv=v1[0];

let mut a=false;
   for i in 1..=v1.len(){
       if v1[i] == 0.0{
       a=false;
       }else {
           a=true;
           sdv=sdv/v1[i];

       }
    }
 
     if a== true{
        None
    } else {
        Some(sdv)
    }
}

 fn avg(v1: Vec<f64>)-> f64{
    
    let sum:f64= v1.iter().sum();
    let l:f64=v1.len() as f64;
    return sum/l;

}

fn sort(v1: &mut Vec<f64>)-> Vec<f64>{
    v1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return v1.to_vec();
}



fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    

   if args.len() < 4 {
        println!("Missing arguments");
        std::process::exit(-1);
    
    } 
     
    let a = args[1].as_str();
    
    let mut nrs = Vec::<f64>::new();
    for i in 2..args.len(){
       let b= args[i].parse::<f64>();
       let x= match b{
       Ok(f)=>f,
       Err(_)=>std::process::exit(-1)
       };
       nrs.push(x)
    }

    match a{
        "add"=>println!("{}",add(nrs)),
     "sub"=>println!("{}",sub(nrs)),
     "mul"=>println!("{}",mul(nrs)),
     "div"=>{
        let d = div(nrs);

    
        match d {
            
            Some(x) => println!("{}", x),
            
            None    => println!("Nothing")
        
    }
  }
  "avg"=>println!("{}",avg(nrs)),
  "sort"=>{
    let d = sort(&mut nrs);
    println!("{:?}", d)
                   
}
_=>print!("Nothing")
    }
    

    
}