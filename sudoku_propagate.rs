extern crate collections;
extern crate time;
use collections::HashMap;
use std::num;
use time::{Timespec, get_time, precise_time_ns};


fn cross(A:&str,B:&str)->Vec<~str>{
    let mut result: Vec<~str> = Vec::new();
    for ai in range(0,A.len()){
        for bi in range(0,B.len()){
            let mut buf = ~"";
            let x = std::str::from_char(A[ai] as char);
            let y = std::str::from_char(B[bi] as char);
            result.push(x+y);
            //println!("{}",x+y);
        }
    }
    //println!("{}",'\n');
    result
}

fn parse_grid(square: Vec<~str>, grid: ~str, digit: ~str, peers: &mut HashMap<~str, HashMap<~str,bool>>,units: &mut HashMap<~str,Vec<~Vec<~str>>>)->~HashMap<~str,Vec<~str>>{
    let mut vec = Vec::new();
    for i in range(1,10){
        vec.push(i.to_str());
    }
    let mut map = ~HashMap::<~str,Vec<~str>>::new();
    for squarei in range(0,square.len()){
        map.insert(square.get(squarei).clone(),vec.clone());
    }
    for i in range(0,square.len()){
        let x = std::str::from_char(grid[i] as char);
        if(!x.eq(&~".") && !assign(map,&square.get(i).clone(),&x,peers,units)){
        }
    }
    return map;
}
fn assign(map: &mut HashMap<~str,Vec<~str>>,sq: &~str, value: &~str,peers: &mut HashMap<~str, HashMap<~str,bool>>,units: &mut HashMap<~str,Vec<~Vec<~str>>>)->bool{
    //println!("HERE in assign");
    let mut result = true;
    let temp = map.get(sq).clone();
    for i in range(0,temp.len()){
        if(!temp.get(i).eq(value)){
            result = eliminate(map,sq,temp.get(i),peers,units);
        }
    }
    return result;
}
//recursive
fn eliminate(map: &mut HashMap<~str,Vec<~str>>,sq: &~str, value: &~str,peers: &mut HashMap<~str, HashMap<~str,bool>>,units: &mut HashMap<~str,Vec<~Vec<~str>>>)->bool{
    //println!("{},{}, elim = {}",sq,map.get(sq),value);
    if(!map.get(sq).contains(value)){//already eliminated
        //println!("return true");
        return true;
    }
    let mut vec = Vec::new();
    for i in range(0,map.get(sq).len()){
        //println!("HERE");
        if(map.get(sq).get(i).eq(value)){
            //map.get_mut(sq).remove(i);
            vec.push(i);
        }
    }
    for j in range(0,vec.len()){
        let temp = vec.get(j);
        //println!("Before {}->{}",sq,map.get(sq));
        if(map.get(sq).len() == 1){
            return false;
        }else{
        map.get_mut(sq).remove(*temp);
        }
        //println!("After {}->{}",sq,map.get(sq));
    }
    //println!("{}->{}",sq,map.get(sq));
    if(map.get(sq).len()==0){
        return false;
    }
    else if(map.get(sq).len() == 1){//remove that number from peers.
        let mut result = true;
        let temp1 = peers.get(sq).clone();
        for (k, v) in temp1.iter() {
            let temp = map.get(sq).clone();
            //println!("len = 1: {}->{}",temp1,map.get(sq));
            //if(map.get(k).len()!=1){
            result = eliminate(map,k,temp.get(0),peers,units);
            //}
        }
        if (!result){
            return false;
        }
    }
    // let temp2 = units.get(sq).clone();
    // for m in range(0,temp2.len()){
    //     let mut tempvec = Vec::new();
    //     for n in range(0,temp2.get(m).len()){//unit 1, unit 2, unit 3
    //         let temp3 = temp2.get(m).get(n);
    //         let val = map.get(temp3);
    //         if(val.len()==1){
    //             if(tempvec.contains(val)){
    //                 return false;
    //             }
    //             else{
    //                 tempvec.push(val.clone());
    //             }
    //         }
    //     }
    // }
    let temp2 = units.get(sq).clone();
    for m in range(0,temp2.len()){
         let mut tempvec = Vec::new();
         for n in range(0,temp2.get(m).len()){
           let temp3 = temp2.get(m).get(n);
            if(map.get(temp3).contains(value)){
                tempvec.push(temp3);
            }
         }
         if(tempvec.len() == 0){
            return false;
         }
         else if(tempvec.len() == 1){
            if(!assign(map,tempvec.get(0).clone(),value,peers,units)){
                return false;
            }
         }
    }
    return true;
}

fn findmin(map: &mut HashMap<~str,Vec<~str>>,square: Vec<~str>)->(uint,uint,~str){
    let mut min = 10;
    let mut max = 1;
    let mut sq = &~"";
    for i in range(0,square.len()){
        let temp = map.get(square.get(i)).len();
        if(temp > max){
            max = map.get(square.get(i)).len();
        }
        if(temp == 2){
            min = temp;
            sq = square.get(i);
            break;
        }
        if(temp>1 && temp < min ){
            min = temp;
            sq = square.get(i);
        }
    }
    return (min,max,sq.clone());
}

fn search(map: &mut HashMap<~str,Vec<~str>>,square: Vec<~str>,peers: &mut HashMap<~str, HashMap<~str,bool>>,units: &mut HashMap<~str,Vec<~Vec<~str>>>){
    let mut stack: Vec<(HashMap<~str,Vec<~str>>,uint,uint)> = Vec::new();
    let (mut min, mut p, mut sq) = findmin(map,square.clone());
    let mut max = 20;
    let mut m = 0;
    stack.push((map.clone(),p,0));
    while(max > 1){
        let (mut min, mut p, mut sq) = findmin(map,square.clone());
        if(p==1){
            break;
        }
        max = p;
        println!("min = {} max = {}, m at {}",min,max,m);
        let length = map.clone();
        if(m >= length.get(&sq).len()){
            loop{
                println!("Backup");
                let (mut tempmap,mut templen, mut tempindex) = stack.pop().unwrap();
                println!("({},{})",tempindex,templen);
                if(tempindex<(templen-1)){
                    *map = tempmap;
                    m = tempindex+1;
                    break;
                }
            }
        }
        let number = map.clone();
        println!("Try {} at ({})",number.get(&sq).get(m),sq);
        stack.push((map.clone(),length.get(&sq).len(),m));
        if(assign(map,&sq,number.get(&sq).get(m),peers,units)){
            m=0;
        }else{
            let (tempmap,templen,tempindex) = stack.pop().unwrap();
            *map = tempmap;
            m = tempindex+1;
            println!("Found conflict try another number");
        }
    }
}


fn main() {
    let digits   = ~"123456789";
    let rows     = ~"ABCDEFGHI";
    let rrows = ~["ABC","DEF","GHI"];
    let ccols = ~["123","456","789"];
    let cols     = digits;
    let mut squares:Vec<~str> = cross(rows, cols);

    let mut vec = Vec::new();
    for i in range(1,10){
        vec.push(i.to_str());
    }
    //initialize unitlist
    let mut unitlist:Vec<~Vec<~str>> = Vec::new();
    for ci in range(0,cols.len()){
        unitlist.push(~cross(rows.clone(), std::str::from_char(cols[ci] as char)));
    }
    for ri in range(0,rows.len()){
        unitlist.push(~cross(std::str::from_char(rows[ri] as char), cols.clone()));
    }
    for rri in range(0, rrows.len()){
        for cci in range(0, ccols.len()){
            unitlist.push(~cross(rrows[rri], ccols[cci]));      
        }
    }

    //initialize units
    let mut units = ~HashMap::new();
    for si in range(0, squares.len()){
        let mut units_tmp : Vec<~Vec<~str>> = Vec::new();
        for ui in range(0, unitlist.len()){
            if(unitlist.get(ui).clone().contains(squares.get(si))){
                units_tmp.push((*unitlist.get(ui)).clone());    
            }
        }
        units.insert(squares.get(si).clone(),units_tmp);
    }
    //println!("{}",units.to_str());
    //peers
    let mut peers = ~HashMap::<~str, HashMap<~str,bool> >::new();
    for i in range(0,squares.len()){
        //let mut temp = ~HashMap::new();
        peers.insert(squares.get(i).clone(),HashMap::new());
        for j in range(0,units.get(squares.get(i)).len()){
            let temp = units.get(squares.get(i)).get(j);
            for k in range(0,temp.len()){
                if(temp.get(k) != squares.get(i)){
                    peers.get_mut(squares.get(i)).insert(temp.get(k).clone(),true);
                }
            }
        }
    }
    //println!("{}",peers.to_str());
    //let testgrid = ~"4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    //easy one with propagation
    let testgrid = ~"..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";
    //let testgrid = ~".....6....59.....82....8....45........3........6..3.54...325..6..................";
    //let testgrid =~"850002400720000009004000000000107002305000900040000000000080070017000000000036040";
    //let testgrid =~"300080000000700005100000000000000360002004000070000000000060130045200000000000800";
    //easy one with propagation
    //let testgrid =~ "200080300060070084030500209000105408000000000402706000301007040720040060004010003";
    println!("Input: ");
    let mut input = ~HashMap::<~str,Vec<~str>>::new();
    for j in range(0,squares.len()){
        let mut tempvec = Vec::new();
        let grid = testgrid.clone();
        let x = std::str::from_char(grid[j] as char);
        tempvec.push(x);
        input.insert(squares.get(j).clone(),tempvec);
    }
    display(input,squares.clone());

    let mut map = parse_grid(squares.clone(),testgrid.clone(),cols.clone(),peers.clone(),units.clone());
    println!("Result of Constraint propagation: ");
    let test = map.clone();
    let mut width = 0;
    let mut height = 0;
    let mut horizon = 0;
    for k in range(0,squares.len()){
        if(width == 3){
            print!("|");
            width = 0;
            height+=1;
        }
        if(height == 3){
            println!{""};
            height = 0;
            horizon +=1;
        }
        if(horizon == 3){
            println!("- - - - -|- - - - -|- - - - -|");
            horizon=0;
        }
        print!(" ");
        for m in range(0,test.get(squares.get(k)).len()){
        print!("{}",test.get(squares.get(k)).get(m).to_str());
        }
        print!(" ");
        width+=1;
        //height+=1;
    }
    print!("|");
    println!("");
    let tv1 = get_time();
    //search(map,squares.clone(),peers.clone(),units.clone());
    let tv2 = get_time();
    //println!("Solved: ");
    //display(map,squares.clone());
    let diff_nsec = tv2.nsec - tv1.nsec;
    //let milisec = diff_nsec/1000000;
    println!("Total time : {} nanosec",diff_nsec);
}
fn display(map: &mut HashMap<~str,Vec<~str>>,squares: Vec<~str>){
    let mut width = 0;
    let mut height = 0;
    let mut horizon = 0;
    for k in range(0,squares.len()){
        if(width == 3){
            print!("|");
            width = 0;
            height+=1;
        }
        if(height == 3){
            println!{""};
            height = 0;
            horizon +=1;
        }
        if(horizon == 3){
            println!("- - - - -|- - - - -|- - - - -|");
            horizon=0;
        }
        print!(" ");
        print!("{}",map.get(squares.get(k)).get(0).to_str());
        print!(" ");
        width+=1;
        //height+=1;
    }
    print!("|");
    println!("");
    // let a = squares[0]; 
}

