use threadpool::ThreadPool;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;
use csv;
use glob::glob;

#[derive(Debug, Clone)]
pub struct ProcessFile{
    total_lines: u64,
    is_member_total: usize,
    total_size: u64
}

impl ProcessFile{
    fn send(self) -> ProcessFile{
        ProcessFile { total_lines: self.total_lines, is_member_total: self.is_member_total, total_size: self.total_size }
    }

    fn default() -> ProcessFile {
        ProcessFile { total_lines: 0, is_member_total: 0, total_size: 0 }

    }
}


//This is how I would parse over 74GB of CSV that has over 19Million rows in just 3 seconds...
//Faith - propenster say so...
pub fn shred() -> Result<(), Box<dyn Error>>{
    let (tx, rx) = channel();

    //prepare the worker pool
    let pool = ThreadPool::new(num_cpus::get());

    //path
    //let path = "./dataset/mtl/*.csv".to_string();
    let path = r#"C:\lib\blockchain-rs\blockchain\src\dataset\mtl\*.csv"#.to_string();

    for path in glob(&path).unwrap().filter_map(Result::ok){
        let tx = tx.clone();
        pool.execute(move || {
            {
                let path = path.display().to_string();

                //generate csv reader here... or whatever you want to do... this is bad...
                let mut reader = csv::Reader::from_path(path).unwrap();

                let mut total = 0;
                let count = reader.records().into_iter().inspect(|_| total += 1).flat_map(|line| line).filter(|line| {
                    line[5].to_string() == "1"
                }).count();

                //sender sends
                tx.send(ProcessFile{
                    total_lines: total,
                    is_member_total: count,
                    total_size: 1000000
                }).unwrap();


            }; // beginning of the actual processing work.... We could refactor this into a different file...
        }) //end of threadPool execute...
    }

    //print....
    let mut process_file = ProcessFile::default();

    rx.into_iter().for_each(|line| {
        //println!("{:?}", line);
        process_file.total_lines += line.total_lines;
        process_file.is_member_total += line.is_member_total;
        process_file.total_size += line.total_size;
    });

    println!("{:?}", process_file);

    Ok(())



}

// fn work(tx: _, path: String) {

    //generate csv reader here... or whatever you want to do...
//     let mut reader = csv::Reader::from_path(path)?;

//     let mut total = 0;
//     let count = reader.records().into_iter().inspect(|_| total += 1).flat_map(|line| line).filter(|line| {
//         line[5].to_string() == "1"
//     }).count();

//     //sender sends
//     tx.send(ProcessFile{
//         total_lines: total,
//         is_member_total: count,
//         total_size: 1000000
//     }).unwrap();


// }