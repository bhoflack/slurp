open Core.Std

let rec walk_dir 
    ~dir 
    ~f =  



let _ = 
    Array.map ~f: (fun s -> print_endline s) (Core.Core_sys.readdir ".")
