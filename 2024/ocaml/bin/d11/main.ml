let parse_input (s : string) : int list =
    Str.split (Str.regexp " ") s
    |> List.map int_of_string


let divide_string (s : string) : string list =
    let len = String.length s in
    let mid = len / 2 in
    [String.sub s 0 mid; String.sub s mid (len - mid)]


let check (n : int) : int list =
    if n == 0 then [1]
    else if (string_of_int n |> String.length) mod 2 == 0 then
        string_of_int n
        |> divide_string
        |> List.map int_of_string
    else [n * 2024]


let rec fold_apply (n : int) (acc : int list) : int list =
    if n <= 0 then acc
    else fold_apply (pred n) (List.map check acc |> List.flatten)


let do_p1 (input : string) (n_runs : int) : int =
    let ns = parse_input input in
    fold_apply n_runs ns
    |> List.length


let () =
    let input = "125 17" in (* Input would go here, but I don't want to commit inputs *)
    Printf.printf "D11P01: %d\n" (do_p1 input 25)