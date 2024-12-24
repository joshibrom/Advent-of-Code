let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s


let parse_input (input : string) : (string list * string list) =
    let parts = Str.split (Str.regexp "\n\n") input in
    (List.nth parts 0 |> Str.split (Str.regexp ", "), List.nth parts 1 |> Str.split (Str.regexp "\n"))


let filter_by_pats (towel : string) (pats : string list) : int =
    List.fold_left (fun acc pat -> 
        Str.split (Str.regexp pat) acc
        |> String.concat ""
    ) towel pats
    |> String.length


let do_p1 (input : string) : int =
    let (pats, towels) = parse_input input in
    let spats = List.sort (fun s1 s2 -> compare (String.length s2) (String.length s1)) pats in
    List.map (fun t -> filter_by_pats t spats) towels
    |> List.filter (fun n -> n == 0)
    |> List.length


let () =
    let input = read_file "inputs/d19.txt" in
    Printf.printf "%d\n" (do_p1 input)