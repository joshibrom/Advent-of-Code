let read_file fname =
    let ch = open_in fname in
    let s = really_input_string ch (in_channel_length ch) in
    close_in ch;
    s

let parse_ln (ln : string) : int list =
    Str.split (Str.regexp " ") ln
    |> List.map int_of_string

let parse_input (input : string) : int list list =
    Str.split (Str.regexp "\n") input
    |> List.map parse_ln

let rec gen_diffs (xs : int list) : int list =
    match xs with
    | a :: b :: ys -> (a - b) :: gen_diffs (b :: ys)
    | _ -> []

let all (bs : bool list) : bool =
    match bs with
    | [] -> true
    | b :: rs -> List.for_all (fun a -> a = b) rs

let is_unidirectional (xs : int list) : bool =
    List.map (fun x -> x > 0) xs
    |> all

let is_bounded (xs: int list) : bool =
    List.map abs xs
    |> List.map (fun x -> x >= 1 && x <= 3)
    |> all

let is_safe (xs : int list) : bool =
    (is_unidirectional xs) && (is_bounded xs)

let do_p1 (input : string) : int =
    parse_input input
    |> List.map gen_diffs
    |> List.filter is_safe
    |> List.length

let () =
    let input = read_file "inputs/d02.txt" in
        Printf.printf "D02P01: %d\n" (do_p1 input);