let input_dir = "../rust/input"

let read_input_to_string (day : int) : string =
  let ch = open_in (Printf.sprintf "%s/d%02d.txt" input_dir day) in
  let s = really_input_string ch (in_channel_length ch) in
  close_in ch;
  s

let read_input_to_lines (day : int) : string list =
  read_input_to_string day
  |> String.split_on_char '\n'
