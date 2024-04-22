let rec solve (pool : int list) : int list list =
  let with_at_least_one_factor =
    List.filter (fun (n : int) -> List.exists (fun x -> (n mod x = 0) && n <> x) pool)
  in
  let without_factors_of (n : int) : int list =
    (* DOES talk of self as factor *)
    List.filter (fun (x : int) -> n mod x <> 0) pool
  in
  let base_case (l : int list list) : int list list =
    if l = [] then [[]] else l
  in
  pool
  |> with_at_least_one_factor  (* only do for numbers with 1+ other factors *)
  |> List.concat_map (fun (choice : int) ->
    choice
    |> without_factors_of  (* new pool *)
    |> solve  (* recursive call *)
    |> base_case  (* if empty list returned, need to add something to it *)
    |> List.map (fun (soln : int list) -> choice :: soln)  (* map new values *)
  )


let _ = solve [1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24]
