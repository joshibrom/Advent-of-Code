let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

let first_int strs = List.hd strs |> int_of_string
let rec last_int strs =
    match strs with
    | [] -> failwith "Empty list given."
    | [s] -> s |> int_of_string
    | _::tl -> last_int tl

let split_to_cols s = Str.split (Str.regexp "   ") s
let c1_from_line line = split_to_cols line |> first_int
let c2_from_line line = split_to_cols line |> last_int
let to_sorted_col s col_selector =
    Str.split (Str.regexp "\n") s
    |> List.map col_selector
    |> List.sort compare

let col1_vals lines = to_sorted_col lines c1_from_line
let col2_vals lines = to_sorted_col lines c2_from_line

let do_p1 s =
    let col1 = col1_vals s in
    let col2 = col2_vals s in
    List.map2 (fun a b -> abs(a - b)) col1 col2
    |> List.fold_left (+) 0

let do_p2 s =
    let col1 = col1_vals s in
    let col2 = col2_vals s in
    List.map
        (fun c1 -> List.find_all (fun c2 -> c1 == c2) col2 |> List.length)
        col1
    |> List.map2 (fun a b -> a * b) col1
    |> List.fold_left (+) 0

let () =
    let input = read_file "inputs/d01.txt" in
        Printf.printf "(OCaml) D01P01: %d\n" (do_p1 input);
        Printf.printf "(OCaml) D01P02: %d\n" (do_p2 input);
