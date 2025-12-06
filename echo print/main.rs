use std::io::{stdin,stdout,Write};

fn main(){

    println!("Echo user input");

    let mut st = String::new();

    loop {
        print!("> ");
        
        stdout()
            .flush()
            .unwrap();
        
        st.clear();
        
        stdin()
            .read_line(&mut st)
            .unwrap();

        let trimed = st.trim_end();

        if trimed == "exit" { break; }
        println!("{trimed}");
    }
}
